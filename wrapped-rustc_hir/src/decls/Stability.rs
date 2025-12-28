macro_rules! deps {
    () => {
        PrintAttribute!();
        StabilityLevel!();
    };
}

macro_rules! Stability {
    () => {
        deps!();
        # [doc = " Represents the following attributes:"] # [doc = ""] # [doc = " - `#[stable]`"] # [doc = " - `#[unstable]`"] # [derive (Encodable , Decodable , Copy , Clone , Debug , PartialEq , Eq , Hash)] # [derive (HashStable_Generic , PrintAttribute)] pub struct Stability { pub level : StabilityLevel , pub feature : Symbol , }
    };
}

Stability!()