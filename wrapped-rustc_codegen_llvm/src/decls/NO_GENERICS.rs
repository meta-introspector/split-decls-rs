macro_rules! deps {
    () => {
        SmallVec!();
        CodegenCx!();
    };
}

macro_rules! NO_GENERICS {
    () => {
        deps!();
        # [doc = " A function that returns an empty list of generic parameter debuginfo nodes."] const NO_GENERICS : for < 'll > fn (& CodegenCx < 'll , '_ >) -> SmallVec < Option < & 'll DIType > > = | _ | SmallVec :: new () ;
    };
}

NO_GENERICS!();