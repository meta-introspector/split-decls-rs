macro_rules! deps {
    () => {
        GroupOps!();
    };
}

macro_rules! GroupOpsOwned {
    () => {
        deps!();
        # [doc = " A helper trait for references with a group operation."] pub trait GroupOpsOwned < Rhs = Self , Output = Self > : for < 'r > GroupOps < & 'r Rhs , Output > { }
    };
}

GroupOpsOwned!()