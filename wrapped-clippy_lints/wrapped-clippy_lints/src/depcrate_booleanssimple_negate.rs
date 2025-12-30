// Generated macro for simple_negate (function)
macro_rules! Depcrate_booleanssimple_negate {
() => {
// Module: crate::booleans
// Provides: {"simple_negate"}
// Dependencies: {}
fn simple_negate (b : Bool) -> Bool { use quine_mc_cluskey :: Bool :: { And , False , Not , Or , Term , True } ; match b { True => False , False => True , t @ Term (_) => Not (Box :: new (t)) , And (mut v) => { for el in & mut v { * el = simple_negate (std :: mem :: replace (el , True)) ; } Or (v) } , Or (mut v) => { for el in & mut v { * el = simple_negate (std :: mem :: replace (el , True)) ; } And (v) } , Not (inner) => * inner , } }
};
}
