// Generated macro for impl_432 (impl)
macro_rules! Depcrate_objects_jmapimpl_432 {
() => {
// Module: crate::objects::jmap
// Provides: {"impl_432"}
// Dependencies: {}
impl < 'local > JMapIter < 'local > { # [doc = " Advances the iterator and returns the next key-value pair in the"] # [doc = " `java.util.Map`, or `None` if there are no more objects."] # [doc = ""] # [doc = " See [`JMap::iter`] for more information."] # [doc = ""] # [doc = " This method creates two new local references. To prevent excessive"] # [doc = " memory usage or overflow error, the local references should be deleted"] # [doc = " using [`Env::delete_local_ref`] or wrapped with"] # [doc = " [`crate::objects::IntoAuto::auto`] before the next loop iteration."] # [doc = " Alternatively, if the map is known to have a small, predictable size,"] # [doc = " the loop could be wrapped in [`Env::with_local_frame`] to delete all of"] # [doc = " the local references at once."] # [doc = ""] # [doc = " This method returns:"] # [doc = ""] # [doc = " * `Ok(Some(_))`: if there was another key-value pair in the map."] # [doc = " * `Ok(None)`: if there are no more key-value pairs in the map."] # [doc = " * `Err(_)`: if there was an error calling the Java method to get the"] # [doc = "   next key-value pair."] # [doc = ""] # [doc = " This is like [`std::iter::Iterator::next`], but requires a parameter of"] # [doc = " type `&mut Env` in order to call into Java."] pub fn next < 'env_local > (& mut self , env : & mut Env < 'env_local > ,) -> Result < Option < JMapEntry < 'env_local > > > { self . iterator . next (env) ? . map_or (Ok (None) , | entry | { let entry = unsafe { JMapEntry :: from_raw (env , entry . into_raw ()) } ; Ok (Some (entry)) }) } }
};
}
