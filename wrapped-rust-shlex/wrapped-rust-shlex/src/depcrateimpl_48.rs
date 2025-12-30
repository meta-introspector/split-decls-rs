// Generated macro for impl_48 (impl)
macro_rules! Depcrateimpl_48 {
() => {
// Module: crate
// Provides: {"impl_48"}
// Dependencies: {}
impl core :: fmt :: Display for QuoteError { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { match self { QuoteError :: Nul => f . write_str ("cannot shell-quote string containing nul byte") , } } }
};
}
