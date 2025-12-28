macro_rules! deps {
    () => {
        Visible!();
        Validators!();
        Deprecation!();
        DefaultValue!();
    };
}

macro_rules! SubscriptionFieldArgument {
    () => {
        deps!();
        # [derive (FromMeta , Default)] # [darling (default)] pub struct SubscriptionFieldArgument { pub name : Option < String > , pub desc : Option < String > , pub default : Option < DefaultValue > , pub default_with : Option < LitStr > , pub validator : Option < Validators > , # [darling (default)] pub process_with : Option < Expr > , pub visible : Option < Visible > , pub secret : bool , pub deprecation : Deprecation , }
    };
}

SubscriptionFieldArgument!();