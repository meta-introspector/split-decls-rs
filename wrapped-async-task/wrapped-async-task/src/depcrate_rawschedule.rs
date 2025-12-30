// Generated macro for schedule (function)
macro_rules! Depcrate_rawschedule {
() => {
// Module: crate::raw
// Provides: {"schedule"}
// Dependencies: {}
# [doc = " Schedules a task for running."] # [doc = ""] # [doc = " This function doesn't modify the state of the task. It only passes the task reference to"] # [doc = " its schedule function."] unsafe fn schedule < S : Schedule < M > , M > (ptr : * const () , info : ScheduleInfo) { let header = ptr as * const Header ; let task_layout = (* header) . vtable . layout_info ; let schedule = ptr . add_byte (task_layout . offset_s) as * mut S ; let _waker ; if mem :: size_of :: < S > () > 0 { _waker = Waker :: from_raw (Header :: clone_waker (ptr)) ; } let task = Runnable :: from_raw (NonNull :: new_unchecked (ptr as * mut ())) ; (* schedule) . schedule (task , info) ; }
};
}
