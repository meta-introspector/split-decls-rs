// Generated macro for modifiers (function)
macro_rules! Depcrate_interfacemodifiers {
() => {
// Module: crate::interface
// Provides: {"modifiers"}
// Dependencies: {}
fn modifiers (func : & Function , name : & str , direction : Direction) -> String { let new_modifier = match & func . kind { FunctionKind :: Method (_) if name == "GetType" => " new" , _ => "" , } ; let static_modifiers = match & func . kind { FunctionKind :: Freestanding | FunctionKind :: Static (_) | FunctionKind :: AsyncFreestanding | FunctionKind :: AsyncStatic (_) => "static" , _ => "" , } ; let abstract_modifier = if direction == Direction :: Export { " abstract" } else { "" } ; let async_modifier = match & func . kind { FunctionKind :: AsyncMethod (_) | FunctionKind :: AsyncFreestanding | FunctionKind :: AsyncStatic (_) if abstract_modifier == "" => { " async" } _ => "" , } ; format ! ("{static_modifiers}{abstract_modifier}{async_modifier}{new_modifier}") }
};
}
