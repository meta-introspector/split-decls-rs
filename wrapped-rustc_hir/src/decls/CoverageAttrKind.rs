macro_rules! deps {
    () => {
        PrintAttribute!();
    };
}

macro_rules! CoverageAttrKind {
    () => {
        deps!();
        # [doc = " Successfully-parsed value of a `#[coverage(..)]` attribute."] # [derive (Copy , Debug , Eq , PartialEq , Encodable , Decodable , Clone)] # [derive (HashStable_Generic , PrintAttribute)] pub enum CoverageAttrKind { On , Off , }
    };
}

CoverageAttrKind!()