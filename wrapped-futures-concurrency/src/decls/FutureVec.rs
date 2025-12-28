macro_rules! FutureVec {
    () => {
        # [doc = " An array of futures which can be dropped in-place, intended to be"] # [doc = " constructed once and then accessed through pin projections."] pub (crate) struct FutureVec < T > { futures : Vec < ManuallyDrop < T > > , }
    };
}

FutureVec!()