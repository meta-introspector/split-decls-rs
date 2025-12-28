macro_rules! deps {
    () => {
        Visible!();
        Deprecation!();
        InterfaceFieldArgument!();
    };
}

macro_rules! InterfaceField {
    () => {
        deps!();
        # [derive (FromMeta)] pub struct InterfaceField { pub name : SpannedValue < String > , pub ty : Type , # [darling (default)] pub method : Option < String > , # [darling (default)] pub desc : Option < String > , # [darling (default , multiple , rename = "arg")] pub args : Vec < InterfaceFieldArgument > , # [darling (default)] pub deprecation : Deprecation , # [darling (default)] pub external : bool , # [darling (default)] pub provides : Option < String > , # [darling (default)] pub requires : Option < String > , # [darling (default)] pub visible : Option < Visible > , # [darling (default)] pub inaccessible : bool , # [darling (default , multiple , rename = "tag")] pub tags : Vec < String > , # [darling (default)] pub shareable : bool , # [darling (default)] pub override_from : Option < String > , # [darling (default , multiple , rename = "directive")] pub directives : Vec < Expr > , # [darling (default , multiple)] pub requires_scopes : Vec < String > , }
    };
}

InterfaceField!();