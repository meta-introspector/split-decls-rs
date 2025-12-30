// Generated macro for impl_583 (impl)
macro_rules! Depcrate_parser_regeximpl_583 {
() => {
// Module: crate::parser::regex
// Provides: {"impl_583"}
// Dependencies: {}
impl < 'a , R , Range > Regex < Range > for & 'a R where R : Regex < Range > , { fn is_match (& self , range : Range) -> bool { (* * self) . is_match (range) } fn find_iter < F > (& self , range : Range) -> (usize , F) where F : FromIterator < Range > , { (* * self) . find_iter (range) } fn captures < F , G > (& self , range : Range) -> (usize , G) where F : FromIterator < Range > , G : FromIterator < F > , { (* * self) . captures (range) } fn as_str (& self) -> & str { (* * self) . as_str () } }
};
}
