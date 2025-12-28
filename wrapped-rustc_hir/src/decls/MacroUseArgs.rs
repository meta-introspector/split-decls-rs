macro_rules! deps {
    () => {
        PrintAttribute!();
    };
}

macro_rules! MacroUseArgs {
    () => {
        deps!();
        # [derive (Encodable , Decodable , Clone , Debug , PartialEq , Eq , Hash)] # [derive (HashStable_Generic , PrintAttribute)] pub enum MacroUseArgs { UseAll , UseSpecific (ThinVec < Ident >) , }
    };
}

MacroUseArgs!();