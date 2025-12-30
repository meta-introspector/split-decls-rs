// Generated macro for implement_nonzero_int (macro)
macro_rules! Depcrate_foreign_core_numimplement_nonzero_int {
() => {
// Module: crate::foreign::core::num
// Provides: {"implement_nonzero_int"}
// Dependencies: {}
macro_rules ! implement_nonzero_int { ($ nonzero : ty , $ int : ty) => { impl <'a > Arbitrary <'a > for $ nonzero { fn arbitrary (u : & mut Unstructured <'a >) -> Result < Self > { match Self :: new (<$ int as Arbitrary <'a >>:: arbitrary (u) ?) { Some (n) => Ok (n) , None => Ok (Self :: new (<$ int >:: MAX) . unwrap ()) , } } # [inline] fn size_hint (depth : usize) -> (usize , Option < usize >) { <$ int as Arbitrary <'a >>:: size_hint (depth) } } } ; }
};
}
