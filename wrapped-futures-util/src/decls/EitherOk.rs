macro_rules! deps {
    () => {
        Either!();
    };
}

macro_rules! EitherOk {
    () => {
        deps!();
        type EitherOk < A , B > = Either < (< A as TryFuture > :: Ok , B) , (< B as TryFuture > :: Ok , A) > ;
    };
}

EitherOk!()