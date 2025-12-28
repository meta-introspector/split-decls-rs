macro_rules! deps {
    () => {
        Span!();
        Flags!();
    };
}

macro_rules! SetFlags {
    () => {
        deps!();
        # [doc = " A group of flags that is not applied to a particular regular expression."] # [derive (Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "arbitrary" , derive (arbitrary :: Arbitrary))] pub struct SetFlags { # [doc = " The span of these flags, including the grouping parentheses."] pub span : Span , # [doc = " The actual sequence of flags."] pub flags : Flags , }
    };
}

SetFlags!()