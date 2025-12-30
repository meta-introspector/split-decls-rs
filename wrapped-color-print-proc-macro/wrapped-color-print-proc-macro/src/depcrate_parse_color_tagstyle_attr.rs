// Generated macro for style_attr (function)
macro_rules! Depcrate_parse_color_tagstyle_attr {
() => {
// Module: crate::parse::color_tag
// Provides: {"style_attr"}
// Dependencies: {}
# [doc = " Parses a style attribute."] fn style_attr (input : Input < '_ >) -> Result < '_ , Change > { let (input , word) = alpha1 (input) ? ; let change = match word { "s" | "strong" | "bold" | "em" => Change :: Bold , "dim" => Change :: Dim , "u" | "underline" => Change :: Underline , "i" | "italic" | "italics" => Change :: Italics , "blink" => Change :: Blink , "strike" => Change :: Strike , "reverse" | "rev" => Change :: Reverse , "conceal" | "hide" => Change :: Conceal , _ => { return Err (Err :: Error (Error :: new (input , ErrorKind :: Tag , None))) } } ; Ok ((input , change)) }
};
}
