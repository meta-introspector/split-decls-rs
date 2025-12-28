macro_rules! deps {
    () => {
        Type!();
    };
}

macro_rules! ExpectedFunction {
    () => {
        deps!();
        # [derive (Debug)] pub struct ExpectedFunction < 'db > { pub call : InFile < ExprOrPatPtr > , pub found : Type < 'db > , }
    };
}

ExpectedFunction!();