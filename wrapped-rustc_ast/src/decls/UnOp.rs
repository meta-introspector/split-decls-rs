macro_rules! deps {
    () => {
        Walkable!();
    };
}

macro_rules! UnOp {
    () => {
        deps!();
        # [doc = " Unary operator."] # [doc = ""] # [doc = " Note that `&data` is not an operator, it's an `AddrOf` expression."] # [derive (Clone , Copy , Debug , PartialEq , Encodable , Decodable , HashStable_Generic , Walkable)] pub enum UnOp { # [doc = " The `*` operator for dereferencing"] Deref , # [doc = " The `!` operator for logical inversion"] Not , # [doc = " The `-` operator for negation"] Neg , }
    };
}

UnOp!()