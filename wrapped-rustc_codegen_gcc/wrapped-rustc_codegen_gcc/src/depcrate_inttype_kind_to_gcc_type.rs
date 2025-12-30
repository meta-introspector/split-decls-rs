// Generated macro for type_kind_to_gcc_type (function)
macro_rules! Depcrate_inttype_kind_to_gcc_type {
() => {
// Module: crate::int
// Provides: {"type_kind_to_gcc_type"}
// Dependencies: {}
fn type_kind_to_gcc_type < I : Interner > (kind : TyKind < I >) -> CType { use rustc_middle :: ty :: IntTy :: * ; use rustc_middle :: ty :: UintTy :: * ; use rustc_middle :: ty :: { Int , Uint } ; match kind { Int (I8) => CType :: Int8t , Int (I16) => CType :: Int16t , Int (I32) => CType :: Int32t , Int (I64) => CType :: Int64t , Int (I128) => CType :: Int128t , Uint (U8) => CType :: UInt8t , Uint (U16) => CType :: UInt16t , Uint (U32) => CType :: UInt32t , Uint (U64) => CType :: UInt64t , Uint (U128) => CType :: UInt128t , _ => unimplemented ! ("Kind: {:?}" , kind) , } }
};
}
