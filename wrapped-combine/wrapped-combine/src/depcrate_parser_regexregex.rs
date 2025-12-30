// Generated macro for Regex (trait)
macro_rules! Depcrate_parser_regexRegex {
() => {
// Module: crate::parser::regex
// Provides: {"Regex"}
// Dependencies: {}
pub trait Regex < Range > { fn is_match (& self , range : Range) -> bool ; fn find_iter < F > (& self , range : Range) -> (usize , F) where F : FromIterator < Range > ; fn captures < F , G > (& self , range : Range) -> (usize , G) where F : FromIterator < Range > , G : FromIterator < F > ; fn as_str (& self) -> & str ; }
};
}
