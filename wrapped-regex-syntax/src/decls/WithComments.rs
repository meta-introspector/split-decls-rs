macro_rules! deps {
    () => {
        Comment!();
        Ast!();
    };
}

macro_rules! WithComments {
    () => {
        deps!();
        # [doc = " An abstract syntax tree for a singular expression along with comments"] # [doc = " found."] # [doc = ""] # [doc = " Comments are not stored in the tree itself to avoid complexity. Each"] # [doc = " comment contains a span of precisely where it occurred in the original"] # [doc = " regular expression."] # [derive (Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "arbitrary" , derive (arbitrary :: Arbitrary))] pub struct WithComments { # [doc = " The actual ast."] pub ast : Ast , # [doc = " All comments found in the original regular expression."] pub comments : Vec < Comment > , }
    };
}

WithComments!();