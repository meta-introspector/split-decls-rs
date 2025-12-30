// Generated macro for Elem (struct)
macro_rules! Depcrate_module_lattice_algebraElem {
() => {
// Module: crate::module_lattice::algebra
// Provides: {"Elem"}
// Dependencies: {}
# [doc = " An `Elem` is a member of the specified prime-order field.  Elements can be added,"] # [doc = " subtracted, multiplied, and negated, and the overloaded operators will ensure both that the"] # [doc = " integer values remain in the field, and that the reductions are done efficiently.  For"] # [doc = " addition and subtraction, a simple conditional subtraction is used; for multiplication,"] # [doc = " Barrett reduction."] # [derive (Copy , Clone , Default , Debug , PartialEq)] pub struct Elem < F : Field > (pub F :: Int) ;
};
}
