macro_rules! LambdaSig {
    () => {
        # [doc = " The `<lambda-sig>` production."] # [doc = ""] # [doc = " ```text"] # [doc = " <lambda-sig> ::= <parameter type>+  # Parameter types or \"v\" if the lambda has no parameters"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq , Eq)] pub struct LambdaSig (Vec < TypeHandle >) ;
    };
}

LambdaSig!();