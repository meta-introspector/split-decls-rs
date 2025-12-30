// Generated macro for catch (function)
macro_rules! Depcrate_paniccatch {
() => {
// Module: crate::panic
// Provides: {"catch"}
// Dependencies: {}
pub fn catch < T , F : FnOnce () -> T > (f : F) -> Option < T > { match LAST_ERROR . try_with (| slot | slot . borrow () . is_some ()) { Ok (true) => return None , Ok (false) => { } Err (_) => { } } match panic :: catch_unwind (AssertUnwindSafe (f)) { Ok (ret) => Some (ret) , Err (e) => { LAST_ERROR . with (| slot | * slot . borrow_mut () = Some (e)) ; None } } }
};
}
