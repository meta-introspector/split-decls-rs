macro_rules! deps {
    () => {
        UnzipOp!();
    };
}

macro_rules! Partition {
    () => {
        deps!();
        # [doc = " An `UnzipOp` that routes items depending on a predicate function."] struct Partition < P > { predicate : P , }
    };
}

Partition!();