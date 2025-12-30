// Generated macro for JMapIter (struct)
macro_rules! Depcrate_objects_jmapJMapIter {
() => {
// Module: crate::objects::jmap
// Provides: {"JMapIter"}
// Dependencies: {}
# [doc = " An iterator over the keys and values in a map. See [`JMap::iter`] for more"] # [doc = " information."] # [doc = ""] # [doc = " This is implemented as a thin wrapper over [`JIterator`] and the only"] # [doc = " difference is that [JMapIter::next] will yield [JMapEntry] values,"] # [doc = " (avoiding the need for a runtime type check, compared to using"] # [doc = " [JIterator::next] followed by [`JMapEntry::cast_local`])."] # [doc = ""] # [doc = " This derefs to [`JIterator`]."] # [derive (Debug)] pub struct JMapIter < 'iter_local > { iterator : JIterator < 'iter_local > , }
};
}
