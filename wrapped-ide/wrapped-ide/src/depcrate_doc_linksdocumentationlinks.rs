// Generated macro for DocumentationLinks (struct)
macro_rules! Depcrate_doc_linksDocumentationLinks {
() => {
// Module: crate::doc_links
// Provides: {"DocumentationLinks"}
// Dependencies: {}
# [doc = " Web and local links to an item's documentation."] # [derive (Default , Debug , Clone , PartialEq , Eq)] pub struct DocumentationLinks { # [doc = " The URL to the documentation on docs.rs."] # [doc = " May not lead anywhere."] pub web_url : Option < String > , # [doc = " The URL to the documentation in the local file system."] # [doc = " May not lead anywhere."] pub local_url : Option < String > , }
};
}
