// Generated macro for impl_62 (impl)
macro_rules! Depcrate_configimpl_62 {
() => {
// Module: crate::config
// Provides: {"impl_62"}
// Dependencies: {}
impl StmtData { pub fn empty () -> & 'static Self { static DEFAULT : OnceLock < StmtData > = OnceLock :: new () ; DEFAULT . get_or_init (StmtData :: default) } pub fn method (& self , key : & str) -> MethodData { let mut data = self . methods . get (key) . cloned () . unwrap_or_default () ; if data . unsafe_ . is_none () { data . unsafe_ = self . unsafe_ ; } data } }
};
}
