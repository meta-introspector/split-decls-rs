// Generated macro for impl_45 (impl)
macro_rules! Depcrate_parserimpl_45 {
() => {
// Module: crate::parser
// Provides: {"impl_45"}
// Dependencies: {}
impl < 'a > Iterator for Chars < 'a > { type Item = char ; fn next (& mut self) -> Option < char > { if ! self . ignore_space { let x = self . c () ; self . advance () ; return x ; } while let Some (c) = self . c () { self . advance () ; match c { '\\' => return match self . c () { Some ('#') => { self . advance () ; Some ('#') } _ => Some ('\\') } , '#' => loop { match self . c () { Some (c) => { self . advance () ; if c == '\n' { break ; } } , None => return None } } , _ => if ! c . is_whitespace () { return Some (c) ; } } } None } }
};
}
