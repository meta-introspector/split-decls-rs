// Generated macro for CopyrightTemplate (struct)
macro_rules! DepcrateCopyrightTemplate {
() => {
// Module: crate
// Provides: {"CopyrightTemplate"}
// Dependencies: {}
# [doc = " The HTML template for the toolchain copyright file"] # [derive (Template)] # [template (path = "COPYRIGHT.html")] struct CopyrightTemplate { in_tree : Node , dependencies : BTreeMap < cargo_metadata :: Package , cargo_metadata :: PackageMetadata > , }
};
}
