// Generated macro for impl_81 (impl)
macro_rules! Depcrate_eqimpl_81 {
() => {
// Module: crate::eq
// Provides: {"impl_81"}
// Dependencies: {}
impl < T > ApproxEq for Option < T > where T : Copy + ApproxEq , { type Margin = < T as ApproxEq > :: Margin ; fn approx_eq < M : Into < Self :: Margin > > (self , other : Self , margin : M) -> bool { let margin = margin . into () ; match (self , other) { (None , None) => true , (Some (slf) , Some (oth)) => slf . approx_eq (oth , margin) , _ => false , } } }
};
}
