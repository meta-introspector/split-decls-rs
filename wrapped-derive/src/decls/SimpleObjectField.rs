macro_rules! deps {
    () => {
        DerivedField!();
        CacheControl!();
        Validators!();
        Visible!();
        DefaultValue!();
        Deprecation!();
    };
}

macro_rules! SimpleObjectField {
    () => {
        deps!();
        # [derive (FromField)] # [darling (attributes (graphql) , forward_attrs (doc))] pub struct SimpleObjectField { pub ident : Option < Ident > , pub ty : Type , pub vis : Visibility , pub attrs : Vec < Attribute > , # [darling (default)] pub skip : bool , # [darling (default)] pub skip_output : bool , # [darling (default)] pub skip_input : bool , # [darling (default)] pub name : Option < String > , # [darling (default)] pub deprecation : Deprecation , # [darling (default)] pub owned : bool , # [darling (default)] pub cache_control : CacheControl , # [darling (default)] pub external : bool , # [darling (default)] pub provides : Option < String > , # [darling (default)] pub requires : Option < String > , # [darling (default)] pub shareable : bool , # [darling (default)] pub inaccessible : bool , # [darling (default , multiple , rename = "tag")] pub tags : Vec < String > , # [darling (default)] pub override_from : Option < String > , # [darling (default)] pub guard : Option < Expr > , # [darling (default)] pub visible : Option < Visible > , # [darling (default , multiple)] pub derived : Vec < DerivedField > , # [darling (default)] pub process_with : Option < Expr > , # [darling (default)] pub default : Option < DefaultValue > , # [darling (default)] pub default_with : Option < LitStr > , # [darling (default)] pub validator : Option < Validators > , # [darling (default)] pub flatten : bool , # [darling (default)] pub secret : bool , # [darling (default , multiple , rename = "directive")] pub directives : Vec < Expr > , pub complexity : Option < Expr > , # [darling (default , multiple)] pub requires_scopes : Vec < String > , }
    };
}

SimpleObjectField!();