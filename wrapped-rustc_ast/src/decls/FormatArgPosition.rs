macro_rules! deps {
    () => {
        FormatArgPositionKind!();
        Walkable!();
    };
}

macro_rules! FormatArgPosition {
    () => {
        deps!();
        # [derive (Clone , Encodable , Decodable , Debug , PartialEq , Eq , Walkable)] pub struct FormatArgPosition { # [doc = " Which argument this position refers to (Ok),"] # [doc = " or would've referred to if it existed (Err)."] # [visitable (ignore)] pub index : Result < usize , usize > , # [doc = " What kind of position this is. See [`FormatArgPositionKind`]."] # [visitable (ignore)] pub kind : FormatArgPositionKind , # [doc = " The span of the name or number."] pub span : Option < Span > , }
    };
}

FormatArgPosition!();