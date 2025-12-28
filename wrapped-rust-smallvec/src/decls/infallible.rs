macro_rules! deps {
    () => {
        CollectionAllocErr!();
    };
}

macro_rules! infallible {
    () => {
        deps!();
        # [inline] fn infallible < T > (result : Result < T , CollectionAllocErr >) -> T { match result { Ok (x) => x , Err (CollectionAllocErr :: CapacityOverflow) => panic ! ("capacity overflow") , Err (CollectionAllocErr :: AllocErr { layout }) => alloc :: alloc :: handle_alloc_error (layout) , } }
    };
}

infallible!()