// Generated macro for BuilderFieldType (enum)
macro_rules! Depcrate_builder_fieldBuilderFieldType {
() => {
// Module: crate::builder_field
// Provides: {"BuilderFieldType"}
// Dependencies: {}
# [doc = " The type of a field in the builder struct"] # [derive (Debug , Clone)] pub enum BuilderFieldType < 'a > { # [doc = " The corresonding builder field will be `Option<field_type>`."] Optional (& 'a syn :: Type) , # [doc = " The corresponding builder field will be just this type"] Precise (& 'a syn :: Type) , # [doc = " The corresponding builder field will be a PhantomData"] # [doc = ""] # [doc = " We do this if if the field is disabled.  We mustn't just completely omit the field from the builder:"] # [doc = " if we did that, the builder might have unused generic parameters (since we copy the generics from"] # [doc = " the target struct).   Using a PhantomData of the original field type provides the right generic usage"] # [doc = " (and the right variance).  The alternative would be to give the user a way to separately control"] # [doc = " the generics of the builder struct, which would be very awkward to use and complex to document."] # [doc = " We could just include the field anyway, as `Option<T>`, but this is wasteful of space, and it"] # [doc = " seems good to explicitly suppress the existence of a variable that won't be set or read."] Phantom (& 'a syn :: Type) , }
};
}
