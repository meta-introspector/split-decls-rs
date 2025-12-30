// Generated macro for at_opts (function)
macro_rules! Depcrateat_opts {
() => {
// Module: crate
// Provides: {"at_opts"}
// Dependencies: {}
# [doc = " Create a new cached handle to the object store with support for additional options."] # [doc = ""] # [doc = " `replacements` is an iterator over pairs of old and new object ids for replacement support."] # [doc = " This means that when asking for object `X`, one will receive object `X-replaced` given an iterator like `Some((X, X-replaced))`."] pub fn at_opts (objects_dir : impl Into < PathBuf > , replacements : impl IntoIterator < Item = (gix_hash :: ObjectId , gix_hash :: ObjectId) > , options : store :: init :: Options ,) -> std :: io :: Result < Handle > { let handle = OwnShared :: new (Store :: at_opts (objects_dir . into () , & mut replacements . into_iter () , options ,) ?) . to_handle () ; Ok (Cache :: from (handle)) }
};
}
