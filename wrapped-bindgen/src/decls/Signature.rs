macro_rules! deps {
    () => {
        Type!();
        Param!();
    };
}

macro_rules! Signature {
    () => {
        deps!();
        # [derive (Clone , Debug , PartialEq , Eq)] pub struct Signature { pub call_flags : MethodCallAttributes , pub return_type : Type , pub params : Vec < Param > , }
    };
}

Signature!()