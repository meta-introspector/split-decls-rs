// Generated macro for MemberName (struct)
macro_rules! Depcrate_astMemberName {
() => {
// Module: crate::ast
// Provides: {"MemberName"}
// Dependencies: {}
# [doc = " In libiberty, Member and DerefMember expressions have special handling."] # [doc = " They parse an `UnqualifiedName` (not an `UnscopedName` as the cxxabi docs"] # [doc = " say) and optionally a `TemplateArgs` if it is present. We can't just parse"] # [doc = " a `Name` or an `UnscopedTemplateName` here because that allows other inputs"] # [doc = " that libiberty does not."] # [derive (Clone , Debug , PartialEq , Eq)] pub struct MemberName (Name) ;
};
}
