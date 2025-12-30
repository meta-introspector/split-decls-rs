// Generated macro for register_functions (function)
macro_rules! Depcrateregister_functions {
() => {
// Module: crate
// Provides: {"register_functions"}
// Dependencies: {}
fn register_functions (bundle : & mut FluentBundle) { bundle . add_function ("STREQ" , | positional , _named | match positional { [FluentValue :: String (a) , FluentValue :: String (b)] => format ! ("{}" , (a == b)) . into () , _ => FluentValue :: Error , }) . expect ("Failed to add a function to the bundle.") ; }
};
}
