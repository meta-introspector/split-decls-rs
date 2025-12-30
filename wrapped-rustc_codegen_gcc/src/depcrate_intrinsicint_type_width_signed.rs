// Generated macro for int_type_width_signed (function)
macro_rules! Depcrate_intrinsicint_type_width_signed {
() => {
// Module: crate::intrinsic
// Provides: {"int_type_width_signed"}
// Dependencies: {}
fn int_type_width_signed < 'gcc , 'tcx > (ty : Ty < 'tcx > , cx : & CodegenCx < 'gcc , 'tcx > ,) -> Option < (u64 , bool) > { match * ty . kind () { ty :: Int (t) => Some ((match t { rustc_middle :: ty :: IntTy :: Isize => u64 :: from (cx . tcx . sess . target . pointer_width) , rustc_middle :: ty :: IntTy :: I8 => 8 , rustc_middle :: ty :: IntTy :: I16 => 16 , rustc_middle :: ty :: IntTy :: I32 => 32 , rustc_middle :: ty :: IntTy :: I64 => 64 , rustc_middle :: ty :: IntTy :: I128 => 128 , } , true ,)) , ty :: Uint (t) => Some ((match t { rustc_middle :: ty :: UintTy :: Usize => u64 :: from (cx . tcx . sess . target . pointer_width) , rustc_middle :: ty :: UintTy :: U8 => 8 , rustc_middle :: ty :: UintTy :: U16 => 16 , rustc_middle :: ty :: UintTy :: U32 => 32 , rustc_middle :: ty :: UintTy :: U64 => 64 , rustc_middle :: ty :: UintTy :: U128 => 128 , } , false ,)) , _ => None , } }
};
}
