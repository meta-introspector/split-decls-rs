// Generated macro for impl_29 (impl)
macro_rules! Depcrateimpl_29 {
() => {
// Module: crate
// Provides: {"impl_29"}
// Dependencies: {}
impl < T , B : BitBlock > BlockIter < T , B > where T : Iterator < Item = B > , { fn from_blocks (mut blocks : T) -> BlockIter < T , B > { let h = blocks . next () . unwrap_or_else (B :: zero) ; BlockIter { tail : blocks , head : h , head_offset : 0 , } } }
};
}
