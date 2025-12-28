macro_rules! deps {
    () => {
        PrintAttribute!();
        StabilityLevel!();
    };
}

macro_rules! ConstStability {
    () => {
        deps!();
        # [doc = " Represents the `#[rustc_const_unstable]` and `#[rustc_const_stable]` attributes."] # [derive (Encodable , Decodable , Copy , Clone , Debug , PartialEq , Eq , Hash)] # [derive (HashStable_Generic , PrintAttribute)] pub struct ConstStability { pub level : StabilityLevel , pub feature : Symbol , # [doc = " whether the function has a `#[rustc_promotable]` attribute"] pub promotable : bool , # [doc = " This is true iff the `const_stable_indirect` attribute is present."] pub const_stable_indirect : bool , }
    };
}

ConstStability!();