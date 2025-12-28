macro_rules! deps {
    () => {
        LambdaSig!();
    };
}

macro_rules! ClosureTypeName {
    () => {
        deps!();
        # [doc = " The `<closure-type-name>` production."] # [doc = ""] # [doc = " ```text"] # [doc = " <closure-type-name> ::= Ul <lambda-sig> E [ <nonnegative number> ] _"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq , Eq)] pub struct ClosureTypeName (LambdaSig , Option < usize >) ;
    };
}

ClosureTypeName!()