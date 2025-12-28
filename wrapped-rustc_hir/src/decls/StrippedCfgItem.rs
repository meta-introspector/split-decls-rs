macro_rules! deps {
    () => {
        CfgEntry!();
    };
}

macro_rules! StrippedCfgItem {
    () => {
        deps!();
        # [derive (Debug , Clone , Encodable , Decodable , HashStable_Generic)] pub struct StrippedCfgItem < ModId = DefId > { pub parent_module : ModId , pub ident : Ident , pub cfg : (CfgEntry , Span) , }
    };
}

StrippedCfgItem!();