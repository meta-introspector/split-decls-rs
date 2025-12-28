macro_rules! deps {
    () => {
        LitKind!();
    };
}

macro_rules! Lit {
    () => {
        deps!();
        # [doc = " A literal token."] # [derive (Clone , Copy , PartialEq , Encodable , Decodable , Debug , HashStable_Generic)] pub struct Lit { pub kind : LitKind , pub symbol : Symbol , pub suffix : Option < Symbol > , }
    };
}

Lit!()