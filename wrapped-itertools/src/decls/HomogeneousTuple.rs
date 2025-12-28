macro_rules! deps {
    () => {
        TupleCollect!();
    };
}

macro_rules! HomogeneousTuple {
    () => {
        deps!();
        # [doc = " Implemented for homogeneous tuples of size up to 12."] pub trait HomogeneousTuple : TupleCollect { }
    };
}

HomogeneousTuple!()