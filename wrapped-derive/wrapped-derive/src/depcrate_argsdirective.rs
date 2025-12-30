// Generated macro for Directive (struct)
macro_rules! Depcrate_argsDirective {
() => {
// Module: crate::args
// Provides: {"Directive"}
// Dependencies: {}
# [derive (FromMeta , Default)] # [darling (default)] pub struct Directive { pub internal : bool , pub name : Option < String > , # [darling (default)] pub name_type : bool , pub visible : Option < Visible > , pub repeatable : bool , pub rename_args : Option < RenameRule > , # [darling (multiple , rename = "location")] pub locations : Vec < DirectiveLocation > , }
};
}
