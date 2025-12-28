macro_rules! deps {
    () => {
        Deprecation!();
        Visible!();
    };
}

macro_rules! EnumItem {
    () => {
        deps!();
        # [derive (FromVariant)] # [darling (attributes (graphql) , forward_attrs (doc))] pub struct EnumItem { pub ident : Ident , pub attrs : Vec < Attribute > , pub fields : Fields < Ignored > , # [darling (default)] pub name : Option < String > , # [darling (default)] pub deprecation : Deprecation , # [darling (default)] pub visible : Option < Visible > , # [darling (default)] pub inaccessible : bool , # [darling (default , multiple , rename = "tag")] pub tags : Vec < String > , # [darling (default , multiple , rename = "directive")] pub directives : Vec < Expr > , }
    };
}

EnumItem!()