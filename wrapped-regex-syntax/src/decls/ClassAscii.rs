macro_rules! deps {
    () => {
        ClassAsciiKind!();
        Span!();
    };
}

macro_rules! ClassAscii {
    () => {
        deps!();
        # [doc = " An ASCII character class."] # [derive (Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "arbitrary" , derive (arbitrary :: Arbitrary))] pub struct ClassAscii { # [doc = " The span of this class."] pub span : Span , # [doc = " The kind of ASCII class."] pub kind : ClassAsciiKind , # [doc = " Whether the class is negated or not. e.g., `[[:alpha:]]` is not negated"] # [doc = " but `[[:^alpha:]]` is."] pub negated : bool , }
    };
}

ClassAscii!();