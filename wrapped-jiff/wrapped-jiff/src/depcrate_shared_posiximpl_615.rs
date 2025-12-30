// Generated macro for impl_615 (impl)
macro_rules! Depcrate_shared_posiximpl_615 {
() => {
// Module: crate::shared::posix
// Provides: {"impl_615"}
// Dependencies: {}
impl < S : AsRef < str > > core :: fmt :: Display for AbbreviationDisplay < S > { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { let s = self . 0 . as_ref () ; if s . chars () . any (| ch | ch == '+' || ch == '-') { write ! (f , "<{s}>") } else { write ! (f , "{s}") } } }
};
}
