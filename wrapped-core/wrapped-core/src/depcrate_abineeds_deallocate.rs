// Generated macro for needs_deallocate (function)
macro_rules! Depcrate_abineeds_deallocate {
() => {
// Module: crate::abi
// Provides: {"needs_deallocate"}
// Dependencies: {}
fn needs_deallocate (resolve : & Resolve , ty : & Type , what : Deallocate) -> bool { match ty { Type :: String => true , Type :: ErrorContext => true , Type :: Id (id) => match & resolve . types [* id] . kind { TypeDefKind :: List (_) => true , TypeDefKind :: Type (t) => needs_deallocate (resolve , t , what) , TypeDefKind :: Handle (Handle :: Own (_)) => what . handles () , TypeDefKind :: Handle (Handle :: Borrow (_)) => false , TypeDefKind :: Resource => false , TypeDefKind :: Record (r) => r . fields . iter () . any (| f | needs_deallocate (resolve , & f . ty , what)) , TypeDefKind :: Tuple (t) => t . types . iter () . any (| t | needs_deallocate (resolve , t , what)) , TypeDefKind :: Variant (t) => t . cases . iter () . filter_map (| t | t . ty . as_ref ()) . any (| t | needs_deallocate (resolve , t , what)) , TypeDefKind :: Option (t) => needs_deallocate (resolve , t , what) , TypeDefKind :: Result (t) => [& t . ok , & t . err] . iter () . filter_map (| t | t . as_ref ()) . any (| t | needs_deallocate (resolve , t , what)) , TypeDefKind :: Flags (_) | TypeDefKind :: Enum (_) => false , TypeDefKind :: Future (_) | TypeDefKind :: Stream (_) => what . handles () , TypeDefKind :: Unknown => unreachable ! () , TypeDefKind :: FixedSizeList (..) => todo ! () , } , Type :: Bool | Type :: U8 | Type :: S8 | Type :: U16 | Type :: S16 | Type :: U32 | Type :: S32 | Type :: U64 | Type :: S64 | Type :: F32 | Type :: F64 | Type :: Char => false , } }
};
}
