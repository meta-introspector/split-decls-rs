macro_rules! deps {
    () => {
        Channel!();
        Action!();
        State!();
        Synchronize!();
        Store!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        impl Channel { pub (crate) fn new (location : Location) -> Self { super :: execution (| execution | { let state = execution . objects . insert (State { msg_cnt : 0 , last_send_access : None , last_recv_access : None , sender_synchronize : Synchronize :: new () , receiver_synchronize : VecDeque :: new () , created : location , }) ; tracing :: trace ! (? state , % location , "mpsc::channel") ; Self { state } }) } pub (crate) fn send (& self , location : Location) { self . state . branch_action (Action :: MsgSend , location) ; super :: execution (| execution | { let state = self . state . get_mut (& mut execution . objects) ; state . msg_cnt = state . msg_cnt . checked_add (1) . expect ("overflow") ; state . sender_synchronize . sync_store (& mut execution . threads , Release) ; state . receiver_synchronize . push_back (state . sender_synchronize) ; if state . msg_cnt == 1 { let thread_id = execution . threads . active_id () ; for (id , thread) in execution . threads . iter_mut () { if id == thread_id { continue ; } let obj = thread . operation . as_ref () . map (| operation | operation . object ()) ; if obj == Some (self . state . erase ()) { thread . set_runnable () ; } } } }) } pub (crate) fn recv (& self , location : Location) { self . state . branch_disable (Action :: MsgRecv , self . is_empty () , location) ; super :: execution (| execution | { let state = self . state . get_mut (& mut execution . objects) ; let thread_id = execution . threads . active_id () ; state . msg_cnt = state . msg_cnt . checked_sub (1) . expect ("expected to be able to read the message") ; let mut synchronize = state . receiver_synchronize . pop_front () . unwrap () ; dbg ! (synchronize . sync_load (& mut execution . threads , Acquire)) ; if state . msg_cnt == 0 { for (id , thread) in execution . threads . iter_mut () { if id == thread_id { continue ; } if let Some (operation) = thread . operation . as_ref () { if operation . object () == self . state . erase () && operation . action () == object :: Action :: Channel (Action :: MsgRecv) { let location = operation . location () ; thread . set_blocked (location) ; } } } } }) } # [doc = " Returns `true` if the channel is currently empty"] pub (crate) fn is_empty (& self) -> bool { super :: execution (| execution | self . get_state (& mut execution . objects) . msg_cnt == 0) } fn get_state < 'a > (& self , objects : & 'a mut object :: Store) -> & 'a mut State { self . state . get_mut (objects) } }
    };
}

impl_108!()