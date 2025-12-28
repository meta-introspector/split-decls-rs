macro_rules! deps {
    () => {
        Deprecation!();
        MetaVisibleFn!();
        MetaDirectiveInvocation!();
    };
}

macro_rules! MetaEnumValue {
    () => {
        deps!();
        # [derive (Clone)] pub struct MetaEnumValue { pub name : String , pub description : Option < String > , pub deprecation : Deprecation , pub visible : Option < MetaVisibleFn > , pub inaccessible : bool , pub tags : Vec < String > , pub directive_invocations : Vec < MetaDirectiveInvocation > , }
    };
}

MetaEnumValue!()