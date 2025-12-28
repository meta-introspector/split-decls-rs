macro_rules! deps {
    () => {
        Walkable!();
    };
}

macro_rules! DelimSpan {
    () => {
        deps!();
        # [derive (Debug , Copy , Clone , PartialEq , Encodable , Decodable , HashStable_Generic , Walkable)] pub struct DelimSpan { pub open : Span , pub close : Span , }
    };
}

DelimSpan!();