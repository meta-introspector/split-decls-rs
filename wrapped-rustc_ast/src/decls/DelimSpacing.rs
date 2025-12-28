macro_rules! deps {
    () => {
        Spacing!();
    };
}

macro_rules! DelimSpacing {
    () => {
        deps!();
        # [derive (Copy , Clone , Debug , PartialEq , Encodable , Decodable , HashStable_Generic)] pub struct DelimSpacing { pub open : Spacing , pub close : Spacing , }
    };
}

DelimSpacing!();