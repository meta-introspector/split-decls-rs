// Generated macro for Type (enum)
macro_rules! Depcrate_tyType {
() => {
// Module: crate::ty
// Provides: {"Type"}
// Dependencies: {}
# [derive (Debug , PartialEq , Clone)] pub enum Type { Void , Bool , Char , I8 , U8 , I16 , U16 , I32 , U32 , I64 , U64 , F32 , F64 , ISize , USize , String , Object , AttributeEnum , Name (TypeName) , Array (Box < Self >) , ArrayRef (Box < Self >) , ConstRef (Box < Self >) , Generic (u16) , PtrMut (Box < Self > , usize) , PtrConst (Box < Self > , usize) , ArrayFixed (Box < Self > , usize) , }
};
}
