macro_rules! FutureArray {
    () => {
        # [doc = " An array of futures which can be dropped in-place, intended to be"] # [doc = " constructed once and then accessed through pin projections."] pub (crate) struct FutureArray < T , const N : usize > { futures : [ManuallyDrop < T > ; N] , }
    };
}

FutureArray!();