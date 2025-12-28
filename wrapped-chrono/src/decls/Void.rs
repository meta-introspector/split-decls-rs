macro_rules! deps {
    () => {
        InternalNumeric!();
        InternalFixed!();
    };
}

macro_rules! Void {
    () => {
        deps!();
        # [doc = " An uninhabited type used for `InternalNumeric` and `InternalFixed` below."] # [derive (Clone , PartialEq , Eq , Hash)] enum Void { }
    };
}

Void!()