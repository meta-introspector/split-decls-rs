// Generated macro for Type (enum)
macro_rules! Depcrate_typesType {
() => {
// Module: crate::types
// Provides: {"Type"}
// Dependencies: {}
# [derive (Clone , Debug , PartialEq , Eq , Hash)] pub enum Type { CppFn (CppFn) , Class (Class) , Interface (Interface) , CppInterface (CppInterface) , Delegate (Delegate) , CppDelegate (CppDelegate) , Enum (Enum) , CppEnum (CppEnum) , Struct (Struct) , CppStruct (CppStruct) , CppConst (CppConst) , Generic (GenericParam) , PtrMut (Box < Self > , usize) , PtrConst (Box < Self > , usize) , ArrayFixed (Box < Self > , usize) , Array (Box < Self >) , ArrayRef (Box < Self >) , ConstRef (Box < Self >) , PrimitiveOrEnum (Box < Self > , Box < Self >) , Void , Bool , Char , I8 , U8 , I16 , U16 , I32 , U32 , I64 , U64 , F32 , F64 , ISize , USize , String , Object , Type , PSTR , PCSTR , PWSTR , PCWSTR , GUID , HRESULT , IUnknown , BSTR , BOOL , }
};
}
