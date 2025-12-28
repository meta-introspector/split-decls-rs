macro_rules! deps {
    () => {
        Span!();
        Literal!();
    };
}

macro_rules! ClassSetRange {
    () => {
        deps!();
        # [doc = " A single character class range in a set."] # [derive (Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "arbitrary" , derive (arbitrary :: Arbitrary))] pub struct ClassSetRange { # [doc = " The span of this range."] pub span : Span , # [doc = " The start of this range."] pub start : Literal , # [doc = " The end of this range."] pub end : Literal , }
    };
}

ClassSetRange!()