// Generated macro for is_prim_type_id (function)
macro_rules! Depcrateis_prim_type_id {
() => {
// Module: crate
// Provides: {"is_prim_type_id"}
// Dependencies: {}
fn is_prim_type_id (resolve : & Resolve , id : TypeId) -> bool { match & resolve . types [id] . kind { TypeDefKind :: List (elem) => is_prim_type (resolve , elem) , TypeDefKind :: Option (ty) => is_prim_type (resolve , ty) , TypeDefKind :: Tuple (tuple) => tuple . types . iter () . all (| ty | is_prim_type (resolve , ty)) , TypeDefKind :: Type (ty) => is_prim_type (resolve , ty) , TypeDefKind :: Record (_) | TypeDefKind :: Resource | TypeDefKind :: Handle (_) | TypeDefKind :: Flags (_) | TypeDefKind :: Variant (_) | TypeDefKind :: Enum (_) | TypeDefKind :: Result (_) | TypeDefKind :: Future (_) | TypeDefKind :: Stream (_) | TypeDefKind :: Unknown => false , TypeDefKind :: FixedSizeList (..) => todo ! () , } }
};
}
