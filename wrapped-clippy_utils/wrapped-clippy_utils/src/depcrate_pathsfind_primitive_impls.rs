// Generated macro for find_primitive_impls (function)
macro_rules! Depcrate_pathsfind_primitive_impls {
() => {
// Module: crate::paths
// Provides: {"find_primitive_impls"}
// Dependencies: {}
fn find_primitive_impls (tcx : TyCtxt < '_ > , name : Symbol) -> & [DefId] { let ty = match name { sym :: bool => SimplifiedType :: Bool , sym :: char => SimplifiedType :: Char , sym :: str => SimplifiedType :: Str , sym :: array => SimplifiedType :: Array , sym :: slice => SimplifiedType :: Slice , sym :: const_ptr => SimplifiedType :: Ptr (Mutability :: Not) , sym :: mut_ptr => SimplifiedType :: Ptr (Mutability :: Mut) , sym :: isize => SimplifiedType :: Int (IntTy :: Isize) , sym :: i8 => SimplifiedType :: Int (IntTy :: I8) , sym :: i16 => SimplifiedType :: Int (IntTy :: I16) , sym :: i32 => SimplifiedType :: Int (IntTy :: I32) , sym :: i64 => SimplifiedType :: Int (IntTy :: I64) , sym :: i128 => SimplifiedType :: Int (IntTy :: I128) , sym :: usize => SimplifiedType :: Uint (UintTy :: Usize) , sym :: u8 => SimplifiedType :: Uint (UintTy :: U8) , sym :: u16 => SimplifiedType :: Uint (UintTy :: U16) , sym :: u32 => SimplifiedType :: Uint (UintTy :: U32) , sym :: u64 => SimplifiedType :: Uint (UintTy :: U64) , sym :: u128 => SimplifiedType :: Uint (UintTy :: U128) , sym :: f32 => SimplifiedType :: Float (FloatTy :: F32) , sym :: f64 => SimplifiedType :: Float (FloatTy :: F64) , _ => return & [] , } ; tcx . incoherent_impls (ty) }
};
}
