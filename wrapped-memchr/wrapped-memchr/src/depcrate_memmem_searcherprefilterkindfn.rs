// Generated macro for PrefilterKindFn (type)
macro_rules! Depcrate_memmem_searcherPrefilterKindFn {
() => {
// Module: crate::memmem::searcher
// Provides: {"PrefilterKindFn"}
// Dependencies: {}
# [doc = " The type of a prefilter function."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " When using a function of this type, callers must ensure that the correct"] # [doc = " function is paired with the value populated in `PrefilterKind` union."] type PrefilterKindFn = unsafe fn (strat : & Prefilter , haystack : & [u8]) -> Option < usize > ;
};
}
