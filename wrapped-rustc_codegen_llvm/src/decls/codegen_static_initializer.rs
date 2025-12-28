macro_rules! deps {
    () => {
        CodegenCx!();
    };
}

macro_rules! codegen_static_initializer {
    () => {
        deps!();
        fn codegen_static_initializer < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , def_id : DefId ,) -> Result < (& 'll Value , ConstAllocation < 'tcx >) , ErrorHandled > { let alloc = cx . tcx . eval_static_initializer (def_id) ? ; Ok ((const_alloc_to_llvm (cx , alloc . inner () , true) , alloc)) }
    };
}

codegen_static_initializer!();