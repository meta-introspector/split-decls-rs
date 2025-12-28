macro_rules! deps {
    () => {
        Span!();
        Ast!();
    };
}

macro_rules! Concat {
    () => {
        deps!();
        # [doc = " A concatenation of regular expressions."] # [derive (Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "arbitrary" , derive (arbitrary :: Arbitrary))] pub struct Concat { # [doc = " The span of this concatenation."] pub span : Span , # [doc = " The concatenation regular expressions."] pub asts : Vec < Ast > , }
    };
}

Concat!()