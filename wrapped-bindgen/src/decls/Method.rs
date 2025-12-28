macro_rules! deps {
    () => {
        TypeMap!();
        Signature!();
    };
}

macro_rules! Method {
    () => {
        deps!();
        # [derive (Clone , Debug , PartialEq , Eq)] pub struct Method { pub def : MethodDef , pub signature : Signature , pub dependencies : TypeMap , }
    };
}

Method!()