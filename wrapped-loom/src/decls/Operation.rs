macro_rules! deps {
    () => {
        Ref!();
        Action!();
    };
}

macro_rules! Operation {
    () => {
        deps!();
        # [derive (Debug , Copy , Clone)] pub (super) struct Operation { obj : Ref , action : Action , location : Location , }
    };
}

Operation!();