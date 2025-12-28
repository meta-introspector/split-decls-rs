macro_rules! deps {
    () => {
        Integer!();
        Wide!();
    };
}

macro_rules! as_wide {
    () => {
        deps!();
        # [doc = " Cast to wide type."] # [inline] fn as_wide < T : Integer > (t : T) -> Wide { Wide :: as_cast (t) }
    };
}

as_wide!()