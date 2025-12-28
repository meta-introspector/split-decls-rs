macro_rules! deps {
    () => {
        Either!();
    };
}

macro_rules! EitherErr {
    () => {
        deps!();
        type EitherErr < A , B > = Either < (< A as TryFuture > :: Error , B) , (< B as TryFuture > :: Error , A) > ;
    };
}

EitherErr!()