// Generated macro for InterfaceFieldArgument (struct)
macro_rules! Depcrate_argsInterfaceFieldArgument {
() => {
// Module: crate::args
// Provides: {"InterfaceFieldArgument"}
// Dependencies: {}
# [derive (FromMeta)] pub struct InterfaceFieldArgument { pub name : String , # [darling (default)] pub desc : Option < String > , pub ty : Type , # [darling (default)] pub default : Option < DefaultValue > , # [darling (default)] pub default_with : Option < LitStr > , # [darling (default)] pub visible : Option < Visible > , # [darling (default)] pub inaccessible : bool , # [darling (default , multiple , rename = "tag")] pub tags : Vec < String > , # [darling (default)] pub secret : bool , # [darling (default , multiple , rename = "directive")] pub directives : Vec < Expr > , # [darling (default)] pub deprecation : Deprecation , }
};
}
