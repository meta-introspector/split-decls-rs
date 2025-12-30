// Generated macro for impl_674 (impl)
macro_rules! Depcrate_read_pubnamesimpl_674 {
() => {
// Module: crate::read::pubnames
// Provides: {"impl_674"}
// Dependencies: {}
impl < R : Reader > PubNamesEntryIter < R > { # [doc = " Advance the iterator and return the next pubname."] # [doc = ""] # [doc = " Returns the newly parsed pubname as `Ok(Some(pubname))`. Returns"] # [doc = " `Ok(None)` when iteration is complete and all pubnames have already been"] # [doc = " parsed and yielded. If an error occurs while parsing the next pubname,"] # [doc = " then this error is returned as `Err(e)`, and all subsequent calls return"] # [doc = " `Ok(None)`."] pub fn next (& mut self) -> Result < Option < PubNamesEntry < R > > > { self . 0 . next () } }
};
}
