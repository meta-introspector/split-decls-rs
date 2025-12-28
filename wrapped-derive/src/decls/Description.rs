macro_rules! Description {
    () => {
        # [derive (FromDeriveInput)] # [darling (forward_attrs (doc))] pub struct Description { pub ident : Ident , pub generics : Generics , pub attrs : Vec < Attribute > , # [darling (default)] pub internal : bool , }
    };
}

Description!();