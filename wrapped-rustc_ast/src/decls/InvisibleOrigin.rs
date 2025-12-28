macro_rules! deps {
    () => {
        MetaVarKind!();
    };
}

macro_rules! InvisibleOrigin {
    () => {
        deps!();
        # [derive (Copy , Clone , PartialEq , Debug , Encodable , Decodable , HashStable_Generic)] pub enum InvisibleOrigin { MetaVar (MetaVarKind) , ProcMacro , }
    };
}

InvisibleOrigin!()