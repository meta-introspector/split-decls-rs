macro_rules! deps {
    () => {
        SimpleObjectField!();
        Resolvability!();
        RenameRule!();
        ConcreteType!();
        Visible!();
        CacheControl!();
    };
}

macro_rules! SimpleObject {
    () => {
        deps!();
        # [derive (FromDeriveInput)] # [darling (attributes (graphql) , forward_attrs (doc))] pub struct SimpleObject { pub ident : Ident , pub generics : Generics , pub attrs : Vec < Attribute > , pub data : Data < Ignored , SimpleObjectField > , # [darling (default)] pub internal : bool , # [darling (default)] pub fake : bool , # [darling (default)] pub complex : bool , # [darling (default)] pub name : Option < String > , # [darling (default)] pub name_type : bool , # [darling (default)] pub rename_fields : Option < RenameRule > , # [darling (default)] pub rename_args : Option < RenameRule > , # [darling (default)] pub cache_control : CacheControl , # [darling (default)] pub extends : bool , # [darling (default)] pub shareable : bool , # [darling (default)] pub inaccessible : bool , # [darling (default)] pub interface_object : bool , # [darling (default , multiple , rename = "tag")] pub tags : Vec < String > , # [darling (default)] pub visible : Option < Visible > , # [darling (default , multiple , rename = "concrete")] pub concretes : Vec < ConcreteType > , # [darling (default)] pub serial : bool , # [darling (default , rename = "unresolvable")] pub resolvability : Resolvability , # [darling (default)] pub input_name : Option < String > , # [darling (default)] pub guard : Option < Expr > , # [darling (default , multiple , rename = "directive")] pub directives : Vec < Expr > , # [darling (default , multiple)] pub requires_scopes : Vec < String > , }
    };
}

SimpleObject!()