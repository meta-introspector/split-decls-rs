// Generated macro for classify_constructor_return_type (function)
macro_rules! Depcrateclassify_constructor_return_type {
() => {
// Module: crate
// Provides: {"classify_constructor_return_type"}
// Dependencies: {}
fn classify_constructor_return_type (resolve : & Resolve , resource_id : TypeId , result : & Option < Type > ,) -> ConstructorReturnType { fn classify (resolve : & Resolve , resource_id : TypeId , result : & Option < Type > ,) -> Option < ConstructorReturnType > { let resource_id = dealias (resolve , resource_id) ; let typedef = match result . as_ref () ? { Type :: Id (id) => & resolve . types [dealias (resolve , * id)] , _ => return None , } ; match & typedef . kind { TypeDefKind :: Handle (Handle :: Own (id)) if dealias (resolve , * id) == resource_id => { Some (ConstructorReturnType :: Self_) } TypeDefKind :: Result (Result_ { ok , err }) => { let ok_typedef = match ok . as_ref () ? { Type :: Id (id) => & resolve . types [dealias (resolve , * id)] , _ => return None , } ; match & ok_typedef . kind { TypeDefKind :: Handle (Handle :: Own (id)) if dealias (resolve , * id) == resource_id => { Some (ConstructorReturnType :: Result { err : * err }) } _ => None , } } _ => None , } } classify (resolve , resource_id , result) . expect ("invalid constructor") }
};
}
