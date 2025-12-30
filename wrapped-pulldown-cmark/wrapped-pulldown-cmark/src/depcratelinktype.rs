// Generated macro for LinkType (enum)
macro_rules! DepcrateLinkType {
() => {
// Module: crate
// Provides: {"LinkType"}
// Dependencies: {}
# [doc = " Type specifier for inline links. See [the Tag::Link](enum.Tag.html#variant.Link) for more information."] # [derive (Clone , Debug , PartialEq , Copy)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] pub enum LinkType { # [doc = " Inline link like `[foo](bar)`"] Inline , # [doc = " Reference link like `[foo][bar]`"] Reference , # [doc = " Reference without destination in the document, but resolved by the broken_link_callback"] ReferenceUnknown , # [doc = " Collapsed link like `[foo][]`"] Collapsed , # [doc = " Collapsed link without destination in the document, but resolved by the broken_link_callback"] CollapsedUnknown , # [doc = " Shortcut link like `[foo]`"] Shortcut , # [doc = " Shortcut without destination in the document, but resolved by the broken_link_callback"] ShortcutUnknown , # [doc = " Autolink like `<http://foo.bar/baz>`"] Autolink , # [doc = " Email address in autolink like `<john@example.org>`"] Email , # [doc = " Wikilink link like `[[foo]]` or `[[foo|bar]]`"] WikiLink { # [doc = " `true` if the wikilink was piped."] # [doc = ""] # [doc = " * `true` - `[[foo|bar]]`"] # [doc = " * `false` - `[[foo]]`"] has_pothole : bool , } , }
};
}
