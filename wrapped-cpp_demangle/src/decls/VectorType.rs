macro_rules! deps {
    () => {
        Expression!();
    };
}

macro_rules! VectorType {
    () => {
        deps!();
        # [doc = " The `<vector-type>` production."] # [doc = ""] # [doc = " ```text"] # [doc = " <vector-type> ::= Dv <number> _ <type>"] # [doc = "               ::= Dv <expression> _ <type>"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq , Eq)] pub enum VectorType { # [doc = " An vector with a number-literal dimension."] DimensionNumber (usize , TypeHandle) , # [doc = " An vector with an expression for its dimension."] DimensionExpression (Expression , TypeHandle) , }
    };
}

VectorType!()