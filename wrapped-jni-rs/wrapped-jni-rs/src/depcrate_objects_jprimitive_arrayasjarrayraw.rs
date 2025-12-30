// Generated macro for AsJArrayRaw (trait)
macro_rules! Depcrate_objects_jprimitive_arrayAsJArrayRaw {
() => {
// Module: crate::objects::jprimitive_array
// Provides: {"AsJArrayRaw"}
// Dependencies: {}
# [doc = " Trait to access the raw `jarray` pointer for types that wrap an array reference"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Implementing this trait will allow a type to be passed to [`Env::get_array_length()`]"] # [doc = " or other JNI APIs that only work with a valid reference to an array (or `null`)"] # [doc = ""] pub unsafe trait AsJArrayRaw < 'local > : AsRef < JObject < 'local > > { # [doc = " Returns the raw JNI pointer as a `jarray`"] fn as_jarray_raw (& self) -> jarray { self . as_ref () . as_raw () as jarray } }
};
}
