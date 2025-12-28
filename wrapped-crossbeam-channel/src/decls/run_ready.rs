macro_rules! deps {
    () => {
        Operation!();
        SelectHandle!();
        Timeout!();
        Context!();
        Selected!();
    };
}

macro_rules! run_ready {
    () => {
        deps!();
        # [doc = " Runs until one of the operations becomes ready, potentially blocking the current thread."] fn run_ready (handles : & mut [(& dyn SelectHandle , usize , * const u8)] , timeout : Timeout , is_biased : bool ,) -> Option < usize > { if handles . is_empty () { match timeout { Timeout :: Now => return None , Timeout :: Never => { utils :: sleep_until (None) ; unreachable ! () ; } Timeout :: At (when) => { utils :: sleep_until (Some (when)) ; return None ; } } } if ! is_biased { utils :: shuffle (handles) ; } loop { let backoff = Backoff :: new () ; loop { for & (handle , i , _) in handles . iter () { if handle . is_ready () { return Some (i) ; } } if backoff . is_completed () { break ; } else { backoff . snooze () ; } } match timeout { Timeout :: Now => return None , Timeout :: Never => { } Timeout :: At (when) => { if Instant :: now () >= when { return None ; } } } let res = Context :: with (| cx | { let mut sel = Selected :: Waiting ; let mut registered_count = 0 ; for (handle , _ , _) in handles . iter_mut () { registered_count += 1 ; let oper = Operation :: hook :: < & dyn SelectHandle > (handle) ; if handle . watch (oper , cx) { sel = match cx . try_select (Selected :: Operation (oper)) { Ok (()) => Selected :: Operation (oper) , Err (s) => s , } ; break ; } sel = cx . selected () ; if sel != Selected :: Waiting { break ; } } if sel == Selected :: Waiting { let mut deadline : Option < Instant > = match timeout { Timeout :: Now => unreachable ! () , Timeout :: Never => None , Timeout :: At (when) => Some (when) , } ; for & (handle , _ , _) in handles . iter () { if let Some (x) = handle . deadline () { deadline = deadline . map (| y | x . min (y)) . or (Some (x)) ; } } sel = cx . wait_until (deadline) ; } for (handle , _ , _) in handles . iter_mut () . take (registered_count) { handle . unwatch (Operation :: hook :: < & dyn SelectHandle > (handle)) ; } match sel { Selected :: Waiting => unreachable ! () , Selected :: Aborted => { } Selected :: Disconnected => { } Selected :: Operation (_) => { for (handle , i , _) in handles . iter_mut () { let oper = Operation :: hook :: < & dyn SelectHandle > (handle) ; if sel == Selected :: Operation (oper) { return Some (* i) ; } } } } None }) ; if res . is_some () { return res ; } } }
    };
}

run_ready!();