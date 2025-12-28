macro_rules! deps {
    () => {
        MergedSubscriptionField!();
        Visible!();
    };
}

macro_rules! MergedSubscription {
    () => {
        deps!();
        # [derive (FromDeriveInput)] # [darling (attributes (graphql) , forward_attrs (doc))] pub struct MergedSubscription { pub ident : Ident , pub generics : Generics , pub attrs : Vec < Attribute > , pub data : Data < Ignored , MergedSubscriptionField > , # [darling (default)] pub internal : bool , # [darling (default)] pub name : Option < String > , # [darling (default)] pub name_type : bool , # [darling (default)] pub visible : Option < Visible > , # [darling (default)] pub extends : bool , }
    };
}

MergedSubscription!();