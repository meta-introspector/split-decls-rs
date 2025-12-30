// Generated macro for other_2 (other)
macro_rules! Depcrateother_2 {
() => {
// Module: crate
// Provides: {"other_2"}
// Dependencies: {}
# [doc = " Here is a duck-typed interface for any JavaScript object that has a `quack`"] # [doc = " method."] # [doc = ""] # [doc = " Note that any attempts to check if an object is a `Quacks` with"] # [doc = " `JsCast::is_instance_of` (i.e. the `instanceof` operator) will fail because"] # [doc = " there is no JS class named `Quacks`."] # [wasm_bindgen] extern "C" { pub type Quacks ; # [wasm_bindgen (structural , method)] pub fn quack (this : & Quacks) -> String ; }
};
}
