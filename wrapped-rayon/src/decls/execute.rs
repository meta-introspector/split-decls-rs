macro_rules! deps {
    () => {
        ParallelIterator!();
        ParallelExtend!();
        UnzipOp!();
    };
}

macro_rules! execute {
    () => {
        deps!();
        # [doc = " Runs an unzip-like operation into default `ParallelExtend` collections."] fn execute < I , OP , FromA , FromB > (pi : I , op : OP) -> (FromA , FromB) where I : ParallelIterator , OP : UnzipOp < I :: Item > , FromA : Default + Send + ParallelExtend < OP :: Left > , FromB : Default + Send + ParallelExtend < OP :: Right > , { let mut a = FromA :: default () ; let mut b = FromB :: default () ; execute_into (& mut a , & mut b , pi , op) ; (a , b) }
    };
}

execute!();