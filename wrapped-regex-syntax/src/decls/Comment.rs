macro_rules! deps {
    () => {
        Span!();
    };
}

macro_rules! Comment {
    () => {
        deps!();
        # [doc = " A comment from a regular expression with an associated span."] # [doc = ""] # [doc = " A regular expression can only contain comments when the `x` flag is"] # [doc = " enabled."] # [derive (Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "arbitrary" , derive (arbitrary :: Arbitrary))] pub struct Comment { # [doc = " The span of this comment, including the beginning `#` and ending `\\n`."] pub span : Span , # [doc = " The comment text, starting with the first character following the `#`"] # [doc = " and ending with the last character preceding the `\\n`."] pub comment : String , }
    };
}

Comment!()