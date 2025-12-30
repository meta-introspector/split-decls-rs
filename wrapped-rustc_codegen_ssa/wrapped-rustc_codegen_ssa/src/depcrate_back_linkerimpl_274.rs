// Generated macro for impl_274 (impl)
macro_rules! Depcrate_back_linkerimpl_274 {
() => {
// Module: crate::back::linker
// Provides: {"impl_274"}
// Dependencies: {}
impl < 'a > L4Bender < 'a > { fn new (cmd : Command , sess : & 'a Session) -> L4Bender < 'a > { L4Bender { cmd , sess , hinted_static : false } } fn hint_static (& mut self) { if ! self . hinted_static { self . link_or_cc_arg ("-static") ; self . hinted_static = true ; } } }
};
}
