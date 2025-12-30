// Generated macro for run (function)
macro_rules! Depcraterun {
() => {
// Module: crate
// Provides: {"run"}
// Dependencies: {}
# [wasm_bindgen (start)] fn run () { log (& format ! ("Hello from {}!" , name ())) ; let x = MyClass :: new () ; assert_eq ! (x . number () , 42) ; x . set_number (10) ; log (& x . render ()) ; }
};
}
