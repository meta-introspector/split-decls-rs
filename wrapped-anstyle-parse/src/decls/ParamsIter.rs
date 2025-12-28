macro_rules! deps {
    () => {
        Params!();
    };
}

macro_rules! ParamsIter {
    () => {
        deps!();
        # [doc = " Immutable subparameter iterator."] pub struct ParamsIter < 'a > { params : & 'a Params , index : usize , }
    };
}

ParamsIter!()