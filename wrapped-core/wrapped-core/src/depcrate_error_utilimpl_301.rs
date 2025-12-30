// Generated macro for impl_301 (impl)
macro_rules! Depcrate_error_utilimpl_301 {
() => {
// Module: crate::error::util
// Provides: {"impl_301"}
// Dependencies: {}
impl < T > Quoted < T > { # [doc = " Creates a new instance with matching open and close strings."] pub fn new (quote : & 'static str , body : T) -> Self { Self { open : quote , body , close : quote , } } # [doc = " Creates a new instance using a backtick as the open and close string."] pub fn backticks (body : T) -> Self { Self :: new ("`" , body) } }
};
}
