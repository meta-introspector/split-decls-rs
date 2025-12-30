// Generated macro for try_intrinsic (function)
macro_rules! Depcrate_intrinsictry_intrinsic {
() => {
// Module: crate::intrinsic
// Provides: {"try_intrinsic"}
// Dependencies: {}
fn try_intrinsic < 'a , 'b , 'gcc , 'tcx > (bx : & 'b mut Builder < 'a , 'gcc , 'tcx > , try_func : RValue < 'gcc > , data : RValue < 'gcc > , _catch_func : RValue < 'gcc > , dest : PlaceRef < 'tcx , RValue < 'gcc > > ,) { if bx . sess () . panic_strategy () == PanicStrategy :: Abort { bx . call (bx . type_void () , None , None , try_func , & [data] , None , None) ; OperandValue :: Immediate (bx . const_i32 (0)) . store (bx , dest) ; } else { if wants_msvc_seh (bx . sess ()) { unimplemented ! () ; } # [cfg (feature = "master")] codegen_gnu_try (bx , try_func , data , _catch_func , dest) ; # [cfg (not (feature = "master"))] unimplemented ! () ; } }
};
}
