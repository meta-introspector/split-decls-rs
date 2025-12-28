macro_rules! deps {
    () => {
        PrintAttribute!();
        StabilityLevel!();
    };
}

macro_rules! DefaultBodyStability {
    () => {
        deps!();
        # [doc = " Represents the `#[rustc_default_body_unstable]` attribute."] # [derive (Encodable , Decodable , Copy , Clone , Debug , PartialEq , Eq , Hash)] # [derive (HashStable_Generic , PrintAttribute)] pub struct DefaultBodyStability { pub level : StabilityLevel , pub feature : Symbol , }
    };
}

DefaultBodyStability!();