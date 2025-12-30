// Generated macro for could_be_const_with_abi (function)
macro_rules! Depcrate_missing_const_for_fncould_be_const_with_abi {
() => {
// Module: crate::missing_const_for_fn
// Provides: {"could_be_const_with_abi"}
// Dependencies: {}
fn could_be_const_with_abi (cx : & LateContext < '_ > , msrv : Msrv , abi : ExternAbi) -> bool { match abi { ExternAbi :: Rust => true , ExternAbi :: C { unwind : false } => msrv . meets (cx , msrvs :: CONST_EXTERN_C_FN) , _ => msrv . meets (cx , msrvs :: CONST_EXTERN_FN) , } }
};
}
