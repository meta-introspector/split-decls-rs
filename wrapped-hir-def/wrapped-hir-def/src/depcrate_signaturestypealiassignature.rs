// Generated macro for TypeAliasSignature (struct)
macro_rules! Depcrate_signaturesTypeAliasSignature {
() => {
// Module: crate::signatures
// Provides: {"TypeAliasSignature"}
// Dependencies: {}
# [derive (Debug , PartialEq , Eq)] pub struct TypeAliasSignature { pub name : Name , pub generic_params : Arc < GenericParams > , pub store : Arc < ExpressionStore > , pub bounds : Box < [TypeBound] > , pub ty : Option < TypeRefId > , pub flags : TypeAliasFlags , }
};
}
