macro_rules! deps {
    () => {
        FnOnce1!();
        MergeResultFn!();
    };
}

macro_rules! impl_1365 {
    () => {
        deps!();
        impl < T > FnOnce1 < Result < T , T > > for MergeResultFn { type Output = T ; fn call_once (self , arg : Result < T , T >) -> Self :: Output { match arg { Ok (x) => x , Err (x) => x , } } }
    };
}

impl_1365!()