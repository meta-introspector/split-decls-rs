// Generated macro for ObjectUrl (struct)
macro_rules! Depcrate_object_urlObjectUrl {
() => {
// Module: crate::object_url
// Provides: {"ObjectUrl"}
// Dependencies: {}
# [doc = " A resource wrapper around [`URL.createObjectURL`] / [`URL.revokeObjectURL`]."] # [doc = ""] # [doc = " A [`Blob`], in particular a [`File`], can be converted to a short URL representing its data with the above methods."] # [doc = " An [`ObjectUrl`] can be cheaply cloned and shared and revokes the underlying URL when the last reference is dropped."] # [doc = ""] # [doc = " Note that multiple urls can be created for the same blob, without being guaranteed to be de-deduplicated."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust,no_run"] # [doc = " use gloo_file::{Blob, ObjectUrl};"] # [doc = ""] # [doc = " let blob = Blob::new(\"hello world\");"] # [doc = " let object_url = ObjectUrl::from(blob);"] # [doc = " ```"] # [doc = ""] # [doc = " [`URL.createObjectURL`]: https://developer.mozilla.org/en-US/docs/Web/API/URL/createObjectURL"] # [doc = " [`URL.revokeObjectURL`]: https://developer.mozilla.org/en-US/docs/Web/API/URL/revokeObjectURL"] # [doc = " [`File`]: crate::File"] # [derive (Clone)] pub struct ObjectUrl { inner : Rc < ObjectUrlAllocation > , }
};
}
