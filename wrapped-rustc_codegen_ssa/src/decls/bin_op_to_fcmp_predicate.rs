macro_rules! deps {
    () => {
        RealPredicate!();
    };
}

macro_rules! bin_op_to_fcmp_predicate {
    () => {
        deps!();
        pub (crate) fn bin_op_to_fcmp_predicate (op : BinOp) -> RealPredicate { match op { BinOp :: Eq => RealPredicate :: RealOEQ , BinOp :: Ne => RealPredicate :: RealUNE , BinOp :: Lt => RealPredicate :: RealOLT , BinOp :: Le => RealPredicate :: RealOLE , BinOp :: Gt => RealPredicate :: RealOGT , BinOp :: Ge => RealPredicate :: RealOGE , op => bug ! ("bin_op_to_fcmp_predicate: expected comparison operator, found {:?}" , op) , } }
    };
}

bin_op_to_fcmp_predicate!()