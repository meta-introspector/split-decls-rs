macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! Array {
    () => {
        deps!();
        # [doc = " Array parameter / pointer"] pub type Array = Rc < Vec < Value > > ;
    };
}

Array!()