// Generated macro for codegen_array_len (function)
macro_rules! Depcrate_basecodegen_array_len {
() => {
// Module: crate::base
// Provides: {"codegen_array_len"}
// Dependencies: {}
fn codegen_array_len < 'tcx > (fx : & mut FunctionCx < '_ , '_ , 'tcx > , place : CPlace < 'tcx >) -> Value { match * place . layout () . ty . kind () { ty :: Array (_elem_ty , len) => { let len = fx . monomorphize (len) . try_to_target_usize (fx . tcx) . expect ("expected monomorphic const in codegen") as i64 ; fx . bcx . ins () . iconst (fx . pointer_type , len) } ty :: Slice (_elem_ty) => place . to_ptr_unsized () . 1 , _ => bug ! ("Rvalue::Len({:?})" , place) , } }
};
}
