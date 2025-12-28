macro_rules! deps {
    () => {
        Ast!();
        Span!();
    };
}

macro_rules! Alternation {
    () => {
        deps!();
        # [doc = " An alternation of regular expressions."] # [derive (Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "arbitrary" , derive (arbitrary :: Arbitrary))] pub struct Alternation { # [doc = " The span of this alternation."] pub span : Span , # [doc = " The alternate regular expressions."] pub asts : Vec < Ast > , }
    };
}

Alternation!();