// Generated macro for Argument (struct)
macro_rules! Depcrate_argsArgument {
() => {
// Module: crate::args
// Provides: {"Argument"}
// Dependencies: {}
# [derive (FromMeta , Default)] # [darling (default)] pub struct Argument { pub name : Option < String > , pub desc : Option < String > , pub default : Option < DefaultValue > , pub default_with : Option < LitStr > , pub validator : Option < Validators > , # [darling (default)] pub process_with : Option < Expr > , pub key : bool , pub visible : Option < Visible > , pub inaccessible : bool , # [darling (multiple , rename = "tag")] pub tags : Vec < String > , pub secret : bool , # [darling (default , multiple , rename = "directive")] pub directives : Vec < Expr > , pub deprecation : Deprecation , }
};
}
