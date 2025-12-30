// Generated macro for payload_and_results (function)
macro_rules! Depcrate_interfacepayload_and_results {
() => {
// Module: crate::interface
// Provides: {"payload_and_results"}
// Dependencies: {}
fn payload_and_results (resolve : & Resolve , ty : Type , with_wit_results : bool ,) -> (Option < Type > , Vec < TypeId >) { if with_wit_results { return (Some (ty) , Vec :: new ()) ; } fn recurse (resolve : & Resolve , ty : Type , results : & mut Vec < TypeId >) -> Option < Type > { if let Type :: Id (id) = ty { if let TypeDefKind :: Result (result) = & resolve . types [id] . kind { results . push (id) ; if let Some (ty) = result . ok { recurse (resolve , ty , results) } else { None } } else { Some (ty) } } else { Some (ty) } } let mut results = Vec :: new () ; let payload = recurse (resolve , ty , & mut results) ; (payload , results) }
};
}
