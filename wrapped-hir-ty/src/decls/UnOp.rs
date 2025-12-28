macro_rules! UnOp {
    () => {
        # [derive (Copy , Clone , Debug , PartialEq , Eq , PartialOrd , Ord , Hash)] pub enum UnOp { # [doc = " The `!` operator for logical inversion"] Not , # [doc = " The `-` operator for negation"] Neg , }
    };
}

UnOp!();