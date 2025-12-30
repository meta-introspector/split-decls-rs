// Generated macro for impl_900 (impl)
macro_rules! Depcrate_output_treeimpl_900 {
() => {
// Module: crate::output::tree
// Provides: {"impl_900"}
// Dependencies: {}
impl TreePart { # [doc = " Turn this tree part into ASCII-licious box drawing characters!"] # [doc = " (Warning: not actually ASCII)"] pub fn ascii_art (self) -> & 'static str { # [rustfmt :: skip] return match self { Self :: Edge => "├── " , Self :: Line => "│   " , Self :: Corner => "└── " , Self :: Blank => "    " , } ; } }
};
}
