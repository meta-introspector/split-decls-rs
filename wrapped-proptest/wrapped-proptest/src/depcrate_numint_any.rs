// Generated macro for int_any (macro)
macro_rules! Depcrate_numint_any {
() => {
// Module: crate::num
// Provides: {"int_any"}
// Dependencies: {}
macro_rules ! int_any { ($ typ : ident , $ int_any : ident) => { # [doc = " Type of the `ANY` constant."] # [derive (Clone , Copy , Debug)] # [must_use = "strategies do nothing unless used"] pub struct Any (()) ; # [doc = " Generates integers with completely arbitrary values, uniformly"] # [doc = " distributed over the whole range."] pub const ANY : Any = Any (()) ; impl Strategy for Any { type Tree = BinarySearch ; type Value = $ typ ; fn new_tree (& self , runner : & mut TestRunner) -> NewTree < Self > { Ok (BinarySearch :: new ($ int_any ! (runner , $ typ))) } } } ; }
};
}
