macro_rules! deps {
    () => {
        ExpressionStore!();
    };
}

macro_rules! SimpleBody {
    () => {
        deps!();
        # [derive (Debug , PartialEq , Eq)] pub struct SimpleBody { pub store : Arc < ExpressionStore > , }
    };
}

SimpleBody!();