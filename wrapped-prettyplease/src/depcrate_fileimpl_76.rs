// Generated macro for impl_76 (impl)
macro_rules! Depcrate_fileimpl_76 {
() => {
// Module: crate::file
// Provides: {"impl_76"}
// Dependencies: {}
impl Printer { pub fn file (& mut self , file : & File) { self . cbox (0) ; if let Some (shebang) = & file . shebang { self . word (shebang . clone ()) ; self . hardbreak () ; } self . inner_attrs (& file . attrs) ; for item in & file . items { self . item (item) ; } self . end () ; } }
};
}
