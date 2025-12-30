// Generated macro for Splice (struct)
macro_rules! Depcrate_collections_vecSplice {
() => {
// Module: crate::collections::vec
// Provides: {"Splice"}
// Dependencies: {}
# [doc = " A splicing iterator for `Vec`."] # [doc = ""] # [doc = " This struct is created by the [`Vec::splice`] method. See its"] # [doc = " documentation for more information."] # [derive (Debug)] pub struct Splice < 'a , 'bump , I > where I : Iterator , I :: Item : 'a + 'bump , { drain : Drain < 'a , 'bump , I :: Item > , replace_with : I , }
};
}
