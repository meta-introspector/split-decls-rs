// Generated macro for is_arg_by_pointer (function)
macro_rules! Depcrateis_arg_by_pointer {
() => {
// Module: crate
// Provides: {"is_arg_by_pointer"}
// Dependencies: {}
pub fn is_arg_by_pointer (resolve : & Resolve , ty : & Type) -> bool { match ty { Type :: Id (id) => match resolve . types [* id] . kind { TypeDefKind :: Type (t) => is_arg_by_pointer (resolve , & t) , TypeDefKind :: Variant (_) => true , TypeDefKind :: Option (_) => true , TypeDefKind :: Result (_) => true , TypeDefKind :: Enum (_) => false , TypeDefKind :: Flags (_) => false , TypeDefKind :: Handle (_) => false , TypeDefKind :: Tuple (_) | TypeDefKind :: Record (_) | TypeDefKind :: List (_) => true , TypeDefKind :: Future (_) => false , TypeDefKind :: Stream (_) => false , TypeDefKind :: Resource => todo ! ("is_arg_by_pointer for resource") , TypeDefKind :: Unknown => unreachable ! () , TypeDefKind :: FixedSizeList (..) => todo ! () , } , Type :: String => true , _ => false , } }
};
}
