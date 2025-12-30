// Generated macro for impl_46 (impl)
macro_rules! Depcrate_parserimpl_46 {
() => {
// Module: crate::parser
// Provides: {"impl_46"}
// Dependencies: {}
impl < 'a > Chars < 'a > { fn new (chars : & [char] , ignore_space : bool) -> Chars { Chars { chars : chars , cur : 0 , ignore_space : ignore_space , } } fn c (& self) -> Option < char > { self . chars . get (self . cur) . map (| & c | c) } fn advance (& mut self) { self . cur = checkadd (self . cur , 1) ; } fn next_count (& mut self) -> usize { self . next () ; self . cur } }
};
}
