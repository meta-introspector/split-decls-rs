// Generated macro for NavigationTarget (struct)
macro_rules! Depcrate_navigation_targetNavigationTarget {
() => {
// Module: crate::navigation_target
// Provides: {"NavigationTarget"}
// Dependencies: {}
# [doc = " `NavigationTarget` represents an element in the editor's UI which you can"] # [doc = " click on to navigate to a particular piece of code."] # [doc = ""] # [doc = " Typically, a `NavigationTarget` corresponds to some element in the source"] # [doc = " code, like a function or a struct, but this is not strictly required."] # [derive (Clone , PartialEq , Eq , Hash)] pub struct NavigationTarget { pub file_id : FileId , # [doc = " Range which encompasses the whole element."] # [doc = ""] # [doc = " Should include body, doc comments, attributes, etc."] # [doc = ""] # [doc = " Clients should use this range to answer \"is the cursor inside the"] # [doc = " element?\" question."] pub full_range : TextRange , # [doc = " A \"most interesting\" range within the `full_range`."] # [doc = ""] # [doc = " Typically, `full_range` is the whole syntax node, including doc"] # [doc = " comments, and `focus_range` is the range of the identifier."] # [doc = ""] # [doc = " Clients should place the cursor on this range when navigating to this target."] # [doc = ""] # [doc = " This range must be contained within [`Self::full_range`]."] pub focus_range : Option < TextRange > , pub name : Symbol , pub kind : Option < SymbolKind > , pub container_name : Option < Symbol > , pub description : Option < String > , pub docs : Option < Documentation > , # [doc = " In addition to a `name` field, a `NavigationTarget` may also be aliased"] # [doc = " In such cases we want a `NavigationTarget` to be accessible by its alias"] pub alias : Option < Symbol > , }
};
}
