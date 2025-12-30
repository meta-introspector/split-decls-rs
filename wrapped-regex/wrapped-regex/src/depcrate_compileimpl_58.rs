// Generated macro for impl_58 (impl)
macro_rules! Depcrate_compileimpl_58 {
() => {
// Module: crate::compile
// Provides: {"impl_58"}
// Dependencies: {}
impl InstHole { fn fill (& self , goto : InstPtr) -> Inst { match * self { InstHole :: Save { slot } => Inst :: Save (InstSave { goto : goto , slot : slot , }) , InstHole :: EmptyLook { look } => Inst :: EmptyLook (InstEmptyLook { goto : goto , look : look , }) , InstHole :: Char { c } => Inst :: Char (InstChar { goto : goto , c : c , }) , InstHole :: Ranges { ref ranges } => Inst :: Ranges (InstRanges { goto : goto , ranges : ranges . clone () , }) , InstHole :: Bytes { start , end } => Inst :: Bytes (InstBytes { goto : goto , start : start , end : end , }) , } } }
};
}
