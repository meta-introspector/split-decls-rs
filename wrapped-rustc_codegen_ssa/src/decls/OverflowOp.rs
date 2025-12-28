macro_rules! OverflowOp {
    () => {
        # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub enum OverflowOp { Add , Sub , Mul , }
    };
}

OverflowOp!()