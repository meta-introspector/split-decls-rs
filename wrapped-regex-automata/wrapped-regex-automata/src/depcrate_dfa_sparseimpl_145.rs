// Generated macro for impl_145 (impl)
macro_rules! Depcrate_dfa_sparseimpl_145 {
() => {
// Module: crate::dfa::sparse
// Provides: {"impl_145"}
// Dependencies: {}
# [doc = " Other routines that work for all `T`."] impl < T > DFA < T > { # [doc = " Set or unset the prefilter attached to this DFA."] # [doc = ""] # [doc = " This is useful when one has deserialized a DFA from `&[u8]`."] # [doc = " Deserialization does not currently include prefilters, so if you"] # [doc = " want prefilter acceleration, you'll need to rebuild it and attach"] # [doc = " it here."] pub fn set_prefilter (& mut self , prefilter : Option < Prefilter >) { self . pre = prefilter } }
};
}
