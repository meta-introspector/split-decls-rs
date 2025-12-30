// Generated macro for macro_109 (macro)
macro_rules! Depcrate_foreign_core_opsmacro_109 {
() => {
// Module: crate::foreign::core::ops
// Provides: {"macro_109"}
// Dependencies: {}
impl_range ! (RangeInclusive < A >, | r : & RangeInclusive < A >| (r . start () . clone () , r . end () . clone ()) , (A , A) , bounded_range (| (a , b) | a ..= b) , | depth | Ok (crate :: size_hint :: and (< A as Arbitrary >:: try_size_hint (depth) ?, < A as Arbitrary >:: try_size_hint (depth) ?,))) ;
};
}
