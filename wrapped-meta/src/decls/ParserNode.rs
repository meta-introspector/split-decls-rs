macro_rules! deps {
    () => {
        ParserExpr!();
    };
}

macro_rules! ParserNode {
    () => {
        deps!();
        # [doc = " The pest grammar node"] # [derive (Clone , Debug , Eq , PartialEq)] pub struct ParserNode < 'i > { # [doc = " The node's expression"] pub expr : ParserExpr < 'i > , # [doc = " The node's span"] pub span : Span < 'i > , }
    };
}

ParserNode!()