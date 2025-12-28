macro_rules! deps {
    () => {
        PrintAttribute!();
        IntType!();
    };
}

macro_rules! ReprAttr {
    () => {
        deps!();
        # [derive (PartialEq , Debug , Encodable , Decodable , Copy , Clone , HashStable_Generic , PrintAttribute)] pub enum ReprAttr { ReprInt (IntType) , ReprRust , ReprC , ReprPacked (Align) , ReprSimd , ReprTransparent , ReprAlign (Align) , }
    };
}

ReprAttr!()