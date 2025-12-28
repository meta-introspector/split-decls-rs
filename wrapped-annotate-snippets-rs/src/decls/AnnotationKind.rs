macro_rules! deps {
    () => {
        Annotation!();
        Snippet!();
    };
}

macro_rules! AnnotationKind {
    () => {
        deps!();
        # [doc = " The type of [`Annotation`] being applied to a [`Snippet`]"] # [derive (Clone , Copy , Debug , PartialEq , Eq , PartialOrd , Ord)] # [non_exhaustive] pub enum AnnotationKind { # [doc = " For showing the source that the [Group's Title][Group::with_title] references"] # [doc = ""] # [doc = " For [`Title`]-less groups, see [`Group::with_level`]"] Primary , # [doc = " Additional context to better understand the [`Primary`][Self::Primary]"] # [doc = " [`Annotation`]"] # [doc = ""] # [doc = " See also [`Renderer::context`]."] # [doc = ""] # [doc = " [`Renderer::context`]: crate::renderer::Renderer"] Context , # [doc = " Prevents the annotated text from getting [folded][Snippet::fold]"] # [doc = ""] # [doc = " By default, [`Snippet`]s will [fold][`Snippet::fold`] (remove) lines"] # [doc = " that do not contain any annotations. [`Visible`][Self::Visible] makes"] # [doc = " it possible to selectively prevent this behavior for specific text,"] # [doc = " allowing context to be preserved without adding any annotation"] # [doc = " characters."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # #[allow(clippy::needless_doctest_main)]"] # [doc = include_str ! ("../examples/struct_name_as_context.rs")] # [doc = " ```"] # [doc = ""] # [doc = include_str ! ("../examples/struct_name_as_context.svg")] # [doc = ""] Visible , }
    };
}

AnnotationKind!()