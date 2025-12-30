// Generated macro for QPath (enum)
macro_rules! Depcrate_hirQPath {
() => {
// Module: crate::hir
// Provides: {"QPath"}
// Dependencies: {}
# [doc = " Represents an optionally `Self`-qualified value/type path or associated extension."] # [doc = ""] # [doc = " To resolve the path to a `DefId`, call [`qpath_res`]."] # [doc = ""] # [doc = " [`qpath_res`]: ../../rustc_middle/ty/struct.TypeckResults.html#method.qpath_res"] # [derive (Debug , Clone , Copy , HashStable_Generic)] pub enum QPath < 'hir > { # [doc = " Path to a definition, optionally \"fully-qualified\" with a `Self`"] # [doc = " type, if the path points to an associated item in a trait."] # [doc = ""] # [doc = " E.g., an unqualified path like `Clone::clone` has `None` for `Self`,"] # [doc = " while `<Vec<T> as Clone>::clone` has `Some(Vec<T>)` for `Self`,"] # [doc = " even though they both have the same two-segment `Clone::clone` `Path`."] Resolved (Option < & 'hir Ty < 'hir > > , & 'hir Path < 'hir >) , # [doc = " Type-related paths (e.g., `<T>::default` or `<T>::Output`)."] # [doc = " Will be resolved by type-checking to an associated item."] # [doc = ""] # [doc = " UFCS source paths can desugar into this, with `Vec::new` turning into"] # [doc = " `<Vec>::new`, and `T::X::Y::method` into `<<<T>::X>::Y>::method`,"] # [doc = " the `X` and `Y` nodes each being a `TyKind::Path(QPath::TypeRelative(..))`."] TypeRelative (& 'hir Ty < 'hir > , & 'hir PathSegment < 'hir >) , # [doc = " Reference to a `#[lang = \"foo\"]` item."] LangItem (LangItem , Span) , }
};
}
