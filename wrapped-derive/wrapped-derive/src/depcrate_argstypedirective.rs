// Generated macro for TypeDirective (struct)
macro_rules! Depcrate_argsTypeDirective {
() => {
// Module: crate::args
// Provides: {"TypeDirective"}
// Dependencies: {}
# [derive (FromMeta , Default)] # [darling (default)] pub struct TypeDirective { pub internal : bool , pub name : Option < String > , # [darling (default)] pub name_type : bool , pub visible : Option < Visible > , pub repeatable : bool , pub rename_args : Option < RenameRule > , # [darling (multiple , rename = "location")] pub locations : Vec < TypeDirectiveLocation > , # [darling (default)] pub composable : Option < String > , }
};
}
