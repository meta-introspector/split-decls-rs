macro_rules! deps {
    () => {
        Context!();
        Operation!();
        Selected!();
        Token!();
        Timeout!();
        SelectHandle!();
    };
}

macro_rules! run_select {
    () => {
        deps!();
        # [doc = " Runs until one of the operations is selected, potentially blocking the current thread."] # [doc = ""] # [doc = " Successful receive operations will have to be followed up by `channel::read()` and successful"] # [doc = " send operations by `channel::write()`."] fn run_select (handles : & mut [(& dyn SelectHandle , usize , * const u8)] , timeout : Timeout , is_biased : bool ,) -> Option < (Token , usize , * const u8) > { if handles . is_empty () { match timeout { Timeout :: Now => return None , Timeout :: Never => { utils :: sleep_until (None) ; unreachable ! () ; } Timeout :: At (when) => { utils :: sleep_until (Some (when)) ; return None ; } } } if ! is_biased { utils :: shuffle (handles) ; } let mut token = Token :: default () ; for & (handle , i , ptr) in handles . iter () { if handle . try_select (& mut token) { return Some ((token , i , ptr)) ; } } loop { let res = Context :: with (| cx | { let mut sel = Selected :: Waiting ; let mut registered_count = 0 ; let mut index_ready = None ; if let Timeout :: Now = timeout { cx . try_select (Selected :: Aborted) . unwrap () ; } for (handle , i , _) in handles . iter_mut () { registered_count += 1 ; if handle . register (Operation :: hook :: < & dyn SelectHandle > (handle) , cx) { sel = match cx . try_select (Selected :: Aborted) { Ok (()) => { index_ready = Some (* i) ; Selected :: Aborted } Err (s) => s , } ; break ; } sel = cx . selected () ; if sel != Selected :: Waiting { break ; } } if sel == Selected :: Waiting { let mut deadline : Option < Instant > = match timeout { Timeout :: Now => return None , Timeout :: Never => None , Timeout :: At (when) => Some (when) , } ; for & (handle , _ , _) in handles . iter () { if let Some (x) = handle . deadline () { deadline = deadline . map (| y | x . min (y)) . or (Some (x)) ; } } sel = cx . wait_until (deadline) ; } for (handle , _ , _) in handles . iter_mut () . take (registered_count) { handle . unregister (Operation :: hook :: < & dyn SelectHandle > (handle)) ; } match sel { Selected :: Waiting => unreachable ! () , Selected :: Aborted => { if let Some (index_ready) = index_ready { for & (handle , i , ptr) in handles . iter () { if i == index_ready && handle . try_select (& mut token) { return Some ((i , ptr)) ; } } } } Selected :: Disconnected => { } Selected :: Operation (_) => { for (handle , i , ptr) in handles . iter_mut () { if sel == Selected :: Operation (Operation :: hook :: < & dyn SelectHandle > (handle)) { if handle . accept (& mut token , cx) { return Some ((* i , * ptr)) ; } } } } } None }) ; if let Some ((i , ptr)) = res { return Some ((token , i , ptr)) ; } for & (handle , i , ptr) in handles . iter () { if handle . try_select (& mut token) { return Some ((token , i , ptr)) ; } } match timeout { Timeout :: Now => return None , Timeout :: Never => { } Timeout :: At (when) => { if Instant :: now () >= when { return None ; } } } } }
    };
}

run_select!();