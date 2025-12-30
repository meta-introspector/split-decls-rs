// Generated macro for impl_127 (impl)
macro_rules! Depcrate_driverimpl_127 {
() => {
// Module: crate::driver
// Provides: {"impl_127"}
// Dependencies: {}
impl Operation { # [doc = " Return a string that identifies the operation. This happens to be the command-names used in long-running processes as well."] pub fn as_str (& self) -> & 'static str { match self { Operation :: Clean => "clean" , Operation :: Smudge => "smudge" , } } }
};
}
