macro_rules! deps {
    () => {
        TupleWindows!();
        HomogeneousTuple!();
    };
}

macro_rules! tuple_windows {
    () => {
        deps!();
        # [doc = " Create a new tuple windows iterator."] pub fn tuple_windows < I , T > (iter : I) -> TupleWindows < I , T > where I : Iterator < Item = T :: Item > , T : HomogeneousTuple , T :: Item : Clone , { TupleWindows { last : None , iter } }
    };
}

tuple_windows!();