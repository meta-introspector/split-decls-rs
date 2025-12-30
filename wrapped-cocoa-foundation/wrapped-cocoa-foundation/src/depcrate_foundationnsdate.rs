// Generated macro for NSDate (trait)
macro_rules! Depcrate_foundationNSDate {
() => {
// Module: crate::foundation
// Provides: {"NSDate"}
// Dependencies: {}
pub trait NSDate : Sized { unsafe fn distantPast (_ : Self) -> id { msg_send ! [class ! (NSDate) , distantPast] } unsafe fn distantFuture (_ : Self) -> id { msg_send ! [class ! (NSDate) , distantFuture] } }
};
}
