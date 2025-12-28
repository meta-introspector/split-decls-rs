macro_rules! deps {
    () => {
        Builder!();
    };
}

macro_rules! catch_unwind_intrinsic {
    () => {
        deps!();
        fn catch_unwind_intrinsic < 'll , 'tcx > (bx : & mut Builder < '_ , 'll , 'tcx > , try_func : & 'll Value , data : & 'll Value , catch_func : & 'll Value , dest : PlaceRef < 'tcx , & 'll Value > ,) { if bx . sess () . panic_strategy () == PanicStrategy :: Abort { let try_func_ty = bx . type_func (& [bx . type_ptr ()] , bx . type_void ()) ; bx . call (try_func_ty , None , None , try_func , & [data] , None , None) ; OperandValue :: Immediate (bx . const_i32 (0)) . store (bx , dest) ; } else if wants_msvc_seh (bx . sess ()) { codegen_msvc_try (bx , try_func , data , catch_func , dest) ; } else if wants_wasm_eh (bx . sess ()) { codegen_wasm_try (bx , try_func , data , catch_func , dest) ; } else if bx . sess () . target . os == "emscripten" { codegen_emcc_try (bx , try_func , data , catch_func , dest) ; } else { codegen_gnu_try (bx , try_func , data , catch_func , dest) ; } }
    };
}

catch_unwind_intrinsic!()