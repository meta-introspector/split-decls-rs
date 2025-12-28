macro_rules! deps {
    () => {
        Any!();
    };
}

macro_rules! Path {
    () => {
        deps!();
        # [doc = " A key that represents a path (to a resource)."] pub type Path = Any < validate :: Path > ;
    };
}

Path!()