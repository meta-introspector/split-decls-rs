// Generated macro for TypeKind (enum)
macro_rules! Depcrate_types_serviceTypeKind {
() => {
// Module: crate::types::service
// Provides: {"TypeKind"}
// Dependencies: {}
# [doc = " A kind of type; scalar, object, enum, etc."] # [derive (Debug , Clone)] pub enum TypeKind { # [doc = " A scalar type."] Scalar , # [doc = " An object type."] Object (ObjectType) , # [doc = " An interface type."] Interface (InterfaceType) , # [doc = " A union type."] Union (UnionType) , # [doc = " An enum type."] Enum (EnumType) , # [doc = " An input object type."] InputObject (InputObjectType) , }
};
}
