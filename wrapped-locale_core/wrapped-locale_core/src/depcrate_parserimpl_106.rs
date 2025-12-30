// Generated macro for impl_106 (impl)
macro_rules! Depcrate_parserimpl_106 {
() => {
// Module: crate::parser
// Provides: {"impl_106"}
// Dependencies: {}
impl < 'a > SubtagIterator < 'a > { pub const fn new (rest : & 'a [u8]) -> Self { Self { remaining : rest , current : Some (skip_before_separator (rest)) , } } pub const fn next_const (mut self) -> (Self , Option < & 'a [u8] >) { let Some (result) = self . current else { return (self , None) ; } ; self . current = if result . len () < self . remaining . len () { self . remaining = unsafe { self . remaining . split_at_unchecked (result . len () + 1) . 1 } ; Some (skip_before_separator (self . remaining)) } else { None } ; (self , Some (result)) } pub const fn peek (& self) -> Option < & 'a [u8] > { self . current } }
};
}
