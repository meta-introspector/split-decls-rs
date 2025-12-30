// Generated macro for gen_fn (function)
macro_rules! Depcrate_intrinsicgen_fn {
() => {
// Module: crate::intrinsic
// Provides: {"gen_fn"}
// Dependencies: {}
# [cfg (feature = "master")] fn gen_fn < 'a , 'gcc , 'tcx > (cx : & 'a CodegenCx < 'gcc , 'tcx > , name : & str , rust_fn_sig : ty :: PolyFnSig < 'tcx > , codegen : & mut dyn FnMut (Builder < 'a , 'gcc , 'tcx >) ,) -> (Type < 'gcc > , Function < 'gcc >) { let fn_abi = cx . fn_abi_of_fn_ptr (rust_fn_sig , ty :: List :: empty ()) ; let return_type = fn_abi . gcc_type (cx) . return_type ; cx . linkage . set (FunctionType :: Internal) ; let func = cx . declare_fn (name , fn_abi) ; cx . set_frame_pointer_type (func) ; cx . apply_target_cpu_attr (func) ; let block = Builder :: append_block (cx , func , "entry-block") ; let bx = Builder :: build (cx , block) ; codegen (bx) ; (return_type , func) }
};
}
