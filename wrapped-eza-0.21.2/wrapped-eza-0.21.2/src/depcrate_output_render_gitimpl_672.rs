// Generated macro for impl_672 (impl)
macro_rules! Depcrate_output_render_gitimpl_672 {
() => {
// Module: crate::output::render::git
// Provides: {"impl_672"}
// Dependencies: {}
impl f :: GitStatus { fn render (self , colours : & dyn Colours) -> ANSIString < 'static > { # [rustfmt :: skip] return match self { Self :: NotModified => colours . not_modified () . paint ("-") , Self :: New => colours . new () . paint ("N") , Self :: Modified => colours . modified () . paint ("M") , Self :: Deleted => colours . deleted () . paint ("D") , Self :: Renamed => colours . renamed () . paint ("R") , Self :: TypeChange => colours . type_change () . paint ("T") , Self :: Ignored => colours . ignored () . paint ("I") , Self :: Conflicted => colours . conflicted () . paint ("U") , } ; } }
};
}
