// Generated macro for FluentType (trait)
macro_rules! Depcrate_typesFluentType {
() => {
// Module: crate::types
// Provides: {"FluentType"}
// Dependencies: {}
# [doc = " Custom types can implement the [`FluentType`] trait in order to generate a string"] # [doc = " value for use in the message generation process."] pub trait FluentType : fmt :: Debug + AnyEq + 'static { # [doc = " Create a clone of the underlying type."] fn duplicate (& self) -> Box < dyn FluentType + Send > ; # [doc = " Convert the custom type into a string value, for instance a custom `DateTime`"] # [doc = " type could return \"Oct. 27, 2022\"."] fn as_string (& self , intls : & intl_memoizer :: IntlLangMemoizer) -> Cow < 'static , str > ; # [doc = " Convert the custom type into a string value, for instance a custom `DateTime`"] # [doc = " type could return \"Oct. 27, 2022\". This operation is provided the threadsafe"] # [doc = " [`IntlLangMemoizer`](intl_memoizer::concurrent::IntlLangMemoizer)."] fn as_string_threadsafe (& self , intls : & intl_memoizer :: concurrent :: IntlLangMemoizer ,) -> Cow < 'static , str > ; }
};
}
