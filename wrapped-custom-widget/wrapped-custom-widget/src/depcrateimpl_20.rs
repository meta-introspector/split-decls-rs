// Generated macro for impl_20 (impl)
macro_rules! Depcrateimpl_20 {
() => {
// Module: crate
// Provides: {"impl_20"}
// Dependencies: {}
impl Button < '_ > { const fn colors (& self) -> (Color , Color , Color , Color) { let theme = self . theme ; match self . state { State :: Normal => (theme . background , theme . text , theme . shadow , theme . highlight) , State :: Selected => (theme . highlight , theme . text , theme . shadow , theme . highlight) , State :: Active => (theme . background , theme . text , theme . highlight , theme . shadow) , } } }
};
}
