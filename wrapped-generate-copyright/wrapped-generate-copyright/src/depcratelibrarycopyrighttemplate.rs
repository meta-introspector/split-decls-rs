// Generated macro for LibraryCopyrightTemplate (struct)
macro_rules! DepcrateLibraryCopyrightTemplate {
() => {
// Module: crate
// Provides: {"LibraryCopyrightTemplate"}
// Dependencies: {}
# [doc = " The HTML template for the library copyright file"] # [derive (Template)] # [template (path = "COPYRIGHT-library.html")] struct LibraryCopyrightTemplate { in_tree : Node , dependencies : BTreeMap < cargo_metadata :: Package , cargo_metadata :: PackageMetadata > , }
};
}
