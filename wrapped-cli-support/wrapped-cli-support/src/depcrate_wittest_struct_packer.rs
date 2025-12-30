// Generated macro for test_struct_packer (function)
macro_rules! Depcrate_wittest_struct_packer {
() => {
// Module: crate::wit
// Provides: {"test_struct_packer"}
// Dependencies: {}
# [test] fn test_struct_packer () { let mut unpacker = StructUnpacker :: new () ; let i32___ = & AdapterType :: I32 ; let double = & AdapterType :: F64 ; let mut read_ty = | ty | unpacker . read_ty (ty) . unwrap () ; assert_eq ! (read_ty (i32___) , 0) ; assert_eq ! (read_ty (i32___) , 1) ; assert_eq ! (read_ty (double) , 2) ; assert_eq ! (read_ty (i32___) , 4) ; assert_eq ! (read_ty (double) , 6) ; }
};
}
