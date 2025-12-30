// Generated macro for impl_47 (impl)
macro_rules! Depcrateimpl_47 {
() => {
// Module: crate
// Provides: {"impl_47"}
// Dependencies: {}
impl State < '_ > { pub fn finalize < F > (mut self , mut formatter : F) -> Result < Self , Error > where F : fmt :: Write , { if self . shortcuts . is_empty () { return Ok (self) ; } formatter . write_str ("\n") ? ; let mut written_shortcuts = HashSet :: new () ; for shortcut in self . shortcuts . drain (..) { if written_shortcuts . contains (& shortcut) { continue ; } write ! (formatter , "\n[{}" , shortcut . 0) ? ; close_link (& shortcut . 1 , & shortcut . 2 , & mut formatter , LinkType :: Shortcut) ? ; written_shortcuts . insert (shortcut) ; } Ok (self) } pub fn is_in_code_block (& self) -> bool { self . code_block . is_some () } # [doc = " Ensure that [`State::newlines_before_start`] is at least as large as"] # [doc = " the provided option value."] fn set_minimum_newlines_before_start (& mut self , option_value : usize) { if self . newlines_before_start < option_value { self . newlines_before_start = option_value } } }
};
}
