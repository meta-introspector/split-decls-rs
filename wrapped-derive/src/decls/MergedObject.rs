macro_rules! deps {
    () => {
        CacheControl!();
        Visible!();
        MergedObjectField!();
    };
}

macro_rules! MergedObject {
    () => {
        deps!();
        # [derive (FromDeriveInput)] # [darling (attributes (graphql) , forward_attrs (doc))] pub struct MergedObject { pub ident : Ident , pub generics : Generics , pub attrs : Vec < Attribute > , pub data : Data < Ignored , MergedObjectField > , # [darling (default)] pub internal : bool , # [darling (default)] pub name : Option < String > , # [darling (default)] pub name_type : bool , # [darling (default)] pub cache_control : CacheControl , # [darling (default)] pub extends : bool , # [darling (default)] pub shareable : bool , # [darling (default)] pub inaccessible : bool , # [darling (default)] pub interface_object : bool , # [darling (default , multiple , rename = "tag")] pub tags : Vec < String > , # [darling (default)] pub visible : Option < Visible > , # [darling (default)] pub serial : bool , # [darling (default , multiple , rename = "directive")] pub directives : Vec < Expr > , }
    };
}

MergedObject!();