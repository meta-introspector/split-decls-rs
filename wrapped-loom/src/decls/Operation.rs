macro_rules! deps {
    () => {
        Action!();
        Ref!();
    };
}

macro_rules! Operation {
    () => {
        deps!();
        # [derive (Debug , Copy , Clone)] pub (super) struct Operation { obj : Ref , action : Action , location : Location , }
    };
}

Operation!()