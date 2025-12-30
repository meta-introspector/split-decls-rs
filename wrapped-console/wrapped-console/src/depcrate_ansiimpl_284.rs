// Generated macro for impl_284 (impl)
macro_rules! Depcrate_ansiimpl_284 {
() => {
// Module: crate::ansi
// Provides: {"impl_284"}
// Dependencies: {}
impl Display for WithoutAnsi < '_ > { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { for (str , is_ansi) in AnsiCodeIterator :: new (self . str) { if ! is_ansi { f . write_str (str) ? ; } } Ok (()) } }
};
}
