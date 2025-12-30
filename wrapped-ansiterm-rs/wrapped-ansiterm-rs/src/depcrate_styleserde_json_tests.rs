// Generated macro for serde_json_tests (module)
macro_rules! Depcrate_styleserde_json_tests {
() => {
// Module: crate::style
// Provides: {"serde_json_tests"}
// Dependencies: {}
# [cfg (test)] # [cfg (feature = "derive_serde_style")] mod serde_json_tests { use super :: { Style , Colour } ; # [test] fn colour_serialization () { let colours = & [Colour :: Red , Colour :: Blue , Colour :: RGB (123 , 123 , 123) , Colour :: Fixed (255) ,] ; assert_eq ! (serde_json :: to_string (& colours) . unwrap () , String :: from ("[\"Red\",\"Blue\",{\"RGB\":[123,123,123]},{\"Fixed\":255}]")) ; } # [test] fn colour_deserialization () { let colours = & [Colour :: Red , Colour :: Blue , Colour :: RGB (123 , 123 , 123) , Colour :: Fixed (255) ,] ; for colour in colours . into_iter () { let serialized = serde_json :: to_string (& colour) . unwrap () ; let deserialized : Colour = serde_json :: from_str (& serialized) . unwrap () ; assert_eq ! (colour , & deserialized) ; } } # [test] fn style_serialization () { let style = Style :: default () ; assert_eq ! (serde_json :: to_string (& style) . unwrap () , "{\"foreground\":null,\"background\":null,\"is_bold\":false,\"is_dimmed\":false,\"is_italic\":false,\"is_underline\":false,\"is_blink\":false,\"is_reverse\":false,\"is_hidden\":false,\"is_strikethrough\":false}" . to_string ()) ; } }
};
}
