// Generated macro for Group (struct)
macro_rules! Depcrate_snippetGroup {
() => {
// Module: crate::snippet
// Provides: {"Group"}
// Dependencies: {}
# [doc = " A [`Title`] with supporting [context][Element] within a [`Report`]"] # [doc = ""] # [doc = " [Decor][crate::renderer::DecorStyle] is used to visually connect [`Element`]s of a `Group`."] # [doc = ""] # [doc = " Generally, you will create separate group's for:"] # [doc = " - New [`Snippet`]s, especially if they need their own [`AnnotationKind::Primary`]"] # [doc = " - Each logically distinct set of [suggestions][Patch`]"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # #[allow(clippy::needless_doctest_main)]"] # [doc = include_str ! ("../examples/highlight_message.rs")] # [doc = " ```"] # [doc = include_str ! ("../examples/highlight_message.svg")] # [derive (Clone , Debug)] pub struct Group < 'a > { pub (crate) primary_level : Level < 'a > , pub (crate) title : Option < Title < 'a > > , pub (crate) elements : Vec < Element < 'a > > , }
};
}
