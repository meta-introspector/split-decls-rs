macro_rules! deps {
    () => {
        Single!();
    };
}

macro_rules! SingleStreamResult {
    () => {
        deps!();
        type SingleStreamResult < St > = Single < Result < < St as TryStream > :: Ok , < St as TryStream > :: Error > > ;
    };
}

SingleStreamResult!();