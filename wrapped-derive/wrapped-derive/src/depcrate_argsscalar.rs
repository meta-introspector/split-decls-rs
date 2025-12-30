// Generated macro for Scalar (struct)
macro_rules! Depcrate_argsScalar {
() => {
// Module: crate::args
// Provides: {"Scalar"}
// Dependencies: {}
# [derive (FromMeta , Default)] # [darling (default)] pub struct Scalar { pub internal : bool , pub name : Option < String > , # [darling (default)] pub name_type : bool , pub use_type_description : bool , pub visible : Option < Visible > , pub inaccessible : bool , # [darling (multiple , rename = "tag")] pub tags : Vec < String > , pub specified_by_url : Option < String > , # [darling (default , multiple)] pub requires_scopes : Vec < String > , }
};
}
