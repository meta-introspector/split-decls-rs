macro_rules! deps {
    () => {
        Visible!();
        CacheControl!();
        DerivedField!();
        Deprecation!();
    };
}

macro_rules! ComplexObjectField {
    () => {
        deps!();
        # [derive (FromMeta , Default)] # [darling (default)] pub struct ComplexObjectField { pub skip : bool , pub name : Option < String > , pub deprecation : Deprecation , pub cache_control : CacheControl , pub external : bool , pub provides : Option < String > , pub requires : Option < String > , pub shareable : bool , pub inaccessible : bool , # [darling (multiple , rename = "tag")] pub tags : Vec < String > , pub override_from : Option < String > , pub guard : Option < Expr > , pub visible : Option < Visible > , pub complexity : Option < Expr > , # [darling (multiple)] pub derived : Vec < DerivedField > , pub flatten : bool , # [darling (default , multiple , rename = "directive")] pub directives : Vec < Expr > , # [darling (default , multiple)] pub requires_scopes : Vec < String > , }
    };
}

ComplexObjectField!()