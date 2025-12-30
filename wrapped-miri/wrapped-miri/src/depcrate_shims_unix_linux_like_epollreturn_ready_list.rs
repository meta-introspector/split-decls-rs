// Generated macro for return_ready_list (function)
macro_rules! Depcrate_shims_unix_linux_like_epollreturn_ready_list {
() => {
// Module: crate::shims::unix::linux_like::epoll
// Provides: {"return_ready_list"}
// Dependencies: {}
# [doc = " Stores the ready list of the `epfd` epoll instance into `events` (which must be an array),"] # [doc = " and the number of returned events into `dest`."] fn return_ready_list < 'tcx > (epfd : & FileDescriptionRef < Epoll > , dest : & MPlaceTy < 'tcx > , events : & MPlaceTy < 'tcx > , ecx : & mut MiriInterpCx < 'tcx > ,) -> InterpResult < 'tcx > { let mut ready_list = epfd . ready_list . mapping . borrow_mut () ; let mut num_of_events : i32 = 0 ; let mut array_iter = ecx . project_array_fields (events) ? ; while let Some (des) = array_iter . next (ecx) ? { if let Some (epoll_event_instance) = ready_list_next (ecx , & mut ready_list) { ecx . write_int_fields_named (& [("events" , epoll_event_instance . events . into ()) , ("u64" , epoll_event_instance . data . into ()) ,] , & des . 1 ,) ? ; ecx . acquire_clock (& epoll_event_instance . clock) ; num_of_events = num_of_events . strict_add (1) ; } else { break ; } } ecx . write_int (num_of_events , dest) ? ; interp_ok (()) }
};
}
