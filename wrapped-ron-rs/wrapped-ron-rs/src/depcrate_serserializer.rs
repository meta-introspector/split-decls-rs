// Generated macro for Serializer (struct)
macro_rules! Depcrate_serSerializer {
() => {
// Module: crate::ser
// Provides: {"Serializer"}
// Dependencies: {}
# [doc = " The RON serializer."] # [doc = ""] # [doc = " You can just use [`to_string`] for deserializing a value."] # [doc = " If you want it pretty-printed, take a look at [`to_string_pretty`]."] pub struct Serializer < W : fmt :: Write > { output : W , pretty : Option < (PrettyConfig , Pretty) > , default_extensions : Extensions , is_empty : Option < bool > , newtype_variant : bool , recursion_limit : Option < usize > , implicit_some_depth : usize , }
};
}
