macro_rules! deps {
    () => {
        Visible!();
        NewTypeName!();
    };
}

macro_rules! NewType {
    () => {
        deps!();
        # [derive (FromDeriveInput)] # [darling (attributes (graphql) , forward_attrs (doc))] pub struct NewType { pub ident : Ident , pub generics : Generics , pub attrs : Vec < Attribute > , pub data : Data < Ignored , syn :: Type > , # [darling (default)] pub internal : bool , # [darling (default)] pub name : NewTypeName , # [darling (default)] pub visible : Option < Visible > , # [darling (default)] pub inaccessible : bool , # [darling (default , multiple , rename = "tag")] pub tags : Vec < String > , # [darling (default)] pub specified_by_url : Option < String > , }
    };
}

NewType!()