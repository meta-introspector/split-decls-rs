// Generated macro for APIStyle (enum)
macro_rules! DepcrateAPIStyle {
() => {
// Module: crate
// Provides: {"APIStyle"}
// Dependencies: {}
# [doc = " Supported API styles for the generated bindings."] # [derive (Default , Debug , Copy , Clone , PartialEq , Eq , Hash)] pub enum APIStyle { # [doc = " Imported functions borrow arguments, while exported functions receive owned arguments. Reduces the allocation overhead for the canonical ABI."] # [default] Asymmetric , # [doc = " Same API for imported and exported functions. Reduces the allocation overhead for symmetric ABI."] Symmetric , }
};
}
