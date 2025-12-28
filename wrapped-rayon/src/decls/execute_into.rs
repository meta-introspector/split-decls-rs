macro_rules! deps {
    () => {
        ParallelIterator!();
        UnzipA!();
        ParallelExtend!();
        UnzipOp!();
    };
}

macro_rules! execute_into {
    () => {
        deps!();
        # [doc = " Runs an unzip-like operation into `ParallelExtend` collections."] fn execute_into < I , OP , FromA , FromB > (a : & mut FromA , b : & mut FromB , pi : I , op : OP) where I : ParallelIterator , OP : UnzipOp < I :: Item > , FromA : Send + ParallelExtend < OP :: Left > , FromB : Send + ParallelExtend < OP :: Right > , { let iter = UnzipA { base : pi , op , b } ; a . par_extend (iter) ; }
    };
}

execute_into!();