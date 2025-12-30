// Generated macro for macro_10108 (macro)
macro_rules! Depcrate_transmutemacro_10108 {
() => {
// Module: crate::transmute
// Provides: {"macro_10108"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for transmutes between collections whose"] # [doc = " types have different ABI, size or alignment."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This is undefined behavior."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " Currently, we cannot know whether a type is a"] # [doc = " collection, so we just lint the ones that come with `std`."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " // different size, therefore likely out-of-bounds memory access"] # [doc = " // You absolutely do not want this in your code!"] # [doc = " unsafe {"] # [doc = "     std::mem::transmute::<_, Vec<u32>>(vec![2_u16])"] # [doc = " };"] # [doc = " ```"] # [doc = ""] # [doc = " You must always iterate, map and collect the values:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " vec![2_u16].into_iter().map(u32::from).collect::<Vec<_>>();"] # [doc = " ```"] # [clippy :: version = "1.40.0"] pub UNSOUND_COLLECTION_TRANSMUTE , correctness , "transmute between collections of layout-incompatible types" }
};
}
