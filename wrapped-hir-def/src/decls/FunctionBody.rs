macro_rules! deps {
    () => {
        PatId!();
        ExpressionStore!();
    };
}

macro_rules! FunctionBody {
    () => {
        deps!();
        # [derive (Debug , PartialEq , Eq)] pub struct FunctionBody { pub store : Arc < ExpressionStore > , pub parameters : Box < [PatId] > , }
    };
}

FunctionBody!();