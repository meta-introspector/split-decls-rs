// Generated macro for impl_21 (impl)
macro_rules! Depcrateimpl_21 {
() => {
// Module: crate
// Provides: {"impl_21"}
// Dependencies: {}
impl Parse for Entry { fn parse (input : ParseStream < '_ >) -> parse :: Result < Entry > { let attrs = input . call (syn :: Attribute :: parse_outer) ? ; let key = input . parse () ? ; input . parse :: < Token ! [=>] > () ? ; let value = input . parse () ? ; Ok (Entry { key , value , attrs }) } }
};
}
