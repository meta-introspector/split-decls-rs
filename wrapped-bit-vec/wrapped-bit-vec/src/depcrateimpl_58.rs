// Generated macro for impl_58 (impl)
macro_rules! Depcrateimpl_58 {
() => {
// Module: crate
// Provides: {"impl_58"}
// Dependencies: {}
impl < B : BitBlock > Iterator for Iter < '_ , B > { type Item = bool ; # [inline] fn next (& mut self) -> Option < bool > { self . range . next () . map (| i | self . bit_vec . get (i) . unwrap ()) } fn nth (& mut self , n : usize) -> Option < Self :: Item > { self . range . nth (n) . and_then (| i | self . bit_vec . get (i)) } fn size_hint (& self) -> (usize , Option < usize >) { self . range . size_hint () } }
};
}
