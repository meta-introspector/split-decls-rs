macro_rules! InactiveCode {
    () => {
        # [derive (Debug , Clone , Eq , PartialEq)] pub struct InactiveCode { pub node : InFile < SyntaxNodePtr > , pub cfg : CfgExpr , pub opts : CfgOptions , }
    };
}

InactiveCode!()