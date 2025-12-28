macro_rules! deps {
    () => {
        UnzipOp!();
    };
}

macro_rules! PartitionMap {
    () => {
        deps!();
        # [doc = " An `UnzipOp` that routes items depending on how they are mapped `Either`."] struct PartitionMap < P > { predicate : P , }
    };
}

PartitionMap!()