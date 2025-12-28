macro_rules! deps {
    () => {
        Builder!();
        CodegenCx!();
        Linkage!();
    };
}

macro_rules! gen_fn {
    () => {
        deps!();
        fn gen_fn < 'a , 'll , 'tcx > (cx : & 'a CodegenCx < 'll , 'tcx > , name : & str , rust_fn_sig : ty :: PolyFnSig < 'tcx > , codegen : & mut dyn FnMut (Builder < 'a , 'll , 'tcx >) ,) -> (& 'll Type , & 'll Value) { let fn_abi = cx . fn_abi_of_fn_ptr (rust_fn_sig , ty :: List :: empty ()) ; let llty = fn_abi . llvm_type (cx) ; let llfn = cx . declare_fn (name , fn_abi , None) ; cx . set_frame_pointer_type (llfn) ; cx . apply_target_cpu_attr (llfn) ; llvm :: set_linkage (llfn , llvm :: Linkage :: InternalLinkage) ; let llbb = Builder :: append_block (cx , llfn , "entry-block") ; let bx = Builder :: build (cx , llbb) ; codegen (bx) ; (llty , llfn) }
    };
}

gen_fn!();