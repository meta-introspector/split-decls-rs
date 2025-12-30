// Generated macro for dealias (function)
macro_rules! Depcratedealias {
() => {
// Module: crate
// Provides: {"dealias"}
// Dependencies: {}
pub fn dealias (resolve : & Resolve , mut id : TypeId) -> TypeId { loop { match & resolve . types [id] . kind { TypeDefKind :: Type (Type :: Id (that_id)) => id = * that_id , _ => break id , } } }
};
}
