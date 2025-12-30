// Generated macro for Built (struct)
macro_rules! Depcrate_buildBuilt {
() => {
// Module: crate::build
// Provides: {"Built"}
// Dependencies: {}
# [doc = " [`Display`]-able IRI build result."] # [doc = ""] # [doc = " The value of this type can generate an IRI using [`From`]/[`Into`] traits or"] # [doc = " [`Display`] trait."] # [doc = ""] # [doc = " # Security consideration"] # [doc = ""] # [doc = " This can be stringified or directly printed by `std::fmt::Display`, but note"] # [doc = " that this `Display` **does not hide the password part**. Be careful **not to"] # [doc = " print the value using `Display for Built<_>` in public context**."] # [doc = ""] # [doc = " [`From`]: `core::convert::From`"] # [doc = " [`Into`]: `core::convert::Into`"] # [doc = " [`Display`]: `core::fmt::Display`"] # [derive (Debug)] pub struct Built < 'a , T : ? Sized > { # [doc = " Builder with the validated content."] builder : Builder < 'a > , # [doc = " Whether the path is absolute."] path_is_absolute : bool , # [doc = " String type."] _ty_str : PhantomData < fn () -> T > , }
};
}
