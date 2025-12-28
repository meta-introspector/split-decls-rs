macro_rules! deps {
    () => {
        Type!();
    };
}

macro_rules! Signature {
    () => {
        deps!();
        # [derive (Debug)] pub struct Signature { pub flags : MethodCallAttributes , pub return_type : Type , pub types : Vec < Type > , }
    };
}

Signature!();