macro_rules! deps {
    () => {
        ParamHint!();
        Signature!();
        ReturnHint!();
        TypeMap!();
    };
}

macro_rules! CppMethod {
    () => {
        deps!();
        # [derive (Clone , Debug)] pub struct CppMethod { pub def : MethodDef , pub signature : Signature , pub dependencies : TypeMap , pub return_hint : ReturnHint , pub param_hints : Vec < ParamHint > , }
    };
}

CppMethod!();