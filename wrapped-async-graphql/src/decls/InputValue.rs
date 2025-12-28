macro_rules! deps {
    () => {
        Directive!();
        TypeRef!();
        Deprecation!();
    };
}

macro_rules! InputValue {
    () => {
        deps!();
        # [doc = " A GraphQL input value type"] # [derive (Debug)] pub struct InputValue { pub (crate) name : String , pub (crate) description : Option < String > , pub (crate) ty : TypeRef , pub (crate) default_value : Option < Value > , pub (crate) inaccessible : bool , pub (crate) tags : Vec < String > , pub (crate) directives : Vec < Directive > , pub (crate) deprecation : Deprecation , }
    };
}

InputValue!();