macro_rules! deps {
    () => {
        Deprecation!();
        Visible!();
    };
}

macro_rules! SubscriptionField {
    () => {
        deps!();
        # [derive (FromMeta , Default)] # [darling (default)] pub struct SubscriptionField { pub skip : bool , pub name : Option < String > , pub deprecation : Deprecation , pub guard : Option < Expr > , pub visible : Option < Visible > , pub complexity : Option < Expr > , # [darling (default , multiple , rename = "directive")] pub directives : Vec < Expr > , }
    };
}

SubscriptionField!();