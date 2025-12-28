macro_rules! deps {
    () => {
        Expression!();
    };
}

macro_rules! ArrayType {
    () => {
        deps!();
        # [doc = " The `<array-type>` production."] # [doc = ""] # [doc = " ```text"] # [doc = " <array-type> ::= A <positive dimension number> _ <element type>"] # [doc = "              ::= A [<dimension expression>] _ <element type>"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq , Eq)] pub enum ArrayType { # [doc = " An array with a number-literal dimension."] DimensionNumber (usize , TypeHandle) , # [doc = " An array with an expression for its dimension."] DimensionExpression (Expression , TypeHandle) , # [doc = " An array with no dimension."] NoDimension (TypeHandle) , }
    };
}

ArrayType!();