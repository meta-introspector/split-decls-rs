macro_rules! deps {
    () => {
        OneofObjectField!();
        Visible!();
        ConcreteType!();
        RenameRule!();
        InterfaceField!();
    };
}

macro_rules! OneofObject {
    () => {
        deps!();
        # [derive (FromDeriveInput)] # [darling (attributes (graphql) , forward_attrs (doc))] pub struct OneofObject { pub ident : Ident , pub generics : Generics , pub attrs : Vec < Attribute > , pub data : Data < OneofObjectField , Ignored > , # [darling (default)] pub internal : bool , # [darling (default)] pub name : Option < String > , # [darling (default)] pub input_name : Option < String > , # [darling (default)] pub name_type : bool , # [darling (default)] pub rename_fields : Option < RenameRule > , # [darling (default)] pub visible : Option < Visible > , # [darling (default)] pub inaccessible : bool , # [darling (default , multiple , rename = "tag")] pub tags : Vec < String > , # [darling (default , multiple , rename = "concrete")] pub concretes : Vec < ConcreteType > , # [darling (default , multiple , rename = "directive")] pub directives : Vec < Expr > , # [darling (default , multiple , rename = "field")] pub fields : Vec < InterfaceField > , }
    };
}

OneofObject!();