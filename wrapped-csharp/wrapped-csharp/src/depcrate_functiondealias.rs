// Generated macro for dealias (function)
macro_rules! Depcrate_functiondealias {
() => {
// Module: crate::function
// Provides: {"dealias"}
// Dependencies: {}
# [doc = " Dereference any number `TypeDefKind::Type` aliases to retrieve the target type."] fn dealias (resolve : & Resolve , mut id : TypeId) -> TypeId { loop { match & resolve . types [id] . kind { TypeDefKind :: Type (Type :: Id (that_id)) => id = * that_id , _ => break id , } } }
};
}
