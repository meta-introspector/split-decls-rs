macro_rules! deps {
    () => {
        Span!();
        ClassSet!();
        ClassSetBinaryOpKind!();
    };
}

macro_rules! ClassSetBinaryOp {
    () => {
        deps!();
        # [doc = " A Unicode character class set operation."] # [derive (Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "arbitrary" , derive (arbitrary :: Arbitrary))] pub struct ClassSetBinaryOp { # [doc = " The span of this operation. e.g., the `a-z--[h-p]` in `[a-z--h-p]`."] pub span : Span , # [doc = " The type of this set operation."] pub kind : ClassSetBinaryOpKind , # [doc = " The left hand side of the operation."] pub lhs : Box < ClassSet > , # [doc = " The right hand side of the operation."] pub rhs : Box < ClassSet > , }
    };
}

ClassSetBinaryOp!();