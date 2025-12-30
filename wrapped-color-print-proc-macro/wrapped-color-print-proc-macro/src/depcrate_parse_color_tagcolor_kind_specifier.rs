// Generated macro for color_kind_specifier (function)
macro_rules! Depcrate_parse_color_tagcolor_kind_specifier {
() => {
// Module: crate::parse::color_tag
// Provides: {"color_kind_specifier"}
// Dependencies: {}
# [doc = " Parses specifiers like `\"bg:\"`."] fn color_kind_specifier (input : Input < '_ >) -> Result < '_ , ColorKind > { check_parser_before_failure (pair (spaced (alpha1) , stag (":")) , terminated (alt ((map (word (alt ((tag ("fg") , tag ("f")))) , | _ | ColorKind :: Foreground) , map (word (alt ((tag ("bg") , tag ("b")))) , | _ | ColorKind :: Background) ,)) , stag (":") ,) , "Unknown specifier, allowed specifiers are \"bg\" or \"fg\" (shortcuts: \"b\" or \"f\")") (input) }
};
}
