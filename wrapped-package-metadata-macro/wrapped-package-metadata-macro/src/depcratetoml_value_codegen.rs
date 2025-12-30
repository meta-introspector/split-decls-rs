// Generated macro for toml_value_codegen (function)
macro_rules! Depcratetoml_value_codegen {
() => {
// Module: crate
// Provides: {"toml_value_codegen"}
// Dependencies: {}
fn toml_value_codegen (value : & Value) -> proc_macro2 :: TokenStream { match value { Value :: String (s) => quote ! { { # s } } , Value :: Integer (i) => quote ! { { # i } } , Value :: Float (f) => quote ! { { # f } } , Value :: Boolean (b) => quote ! { { # b } } , Value :: Array (a) => toml_array_codegen (a) , Value :: Datetime (d) => { let date_str = toml :: ser :: to_string (d) . unwrap () ; quote ! { { # date_str } } } Value :: Table (_) => { panic ! ("Tables are not supported") ; } } }
};
}
