// Generated macro for impl_664 (impl)
macro_rules! Depcrate_output_render_filetypeimpl_664 {
() => {
// Module: crate::output::render::filetype
// Provides: {"impl_664"}
// Dependencies: {}
impl f :: Type { pub fn render < C : Colours > (self , colours : & C) -> ANSIString < 'static > { # [rustfmt :: skip] return match self { Self :: File => colours . normal () . paint (".") , Self :: Directory => colours . directory () . paint ("d") , Self :: Pipe => colours . pipe () . paint ("|") , Self :: Link => colours . symlink () . paint ("l") , Self :: BlockDevice => colours . block_device () . paint ("b") , Self :: CharDevice => colours . char_device () . paint ("c") , Self :: Socket => colours . socket () . paint ("s") , Self :: Special => colours . special () . paint ("?") , } ; } }
};
}
