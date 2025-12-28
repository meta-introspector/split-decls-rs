macro_rules! deps {
    () => {
        ArcInner!();
        HeaderSlice!();
    };
}

macro_rules! thin_to_thick {
    () => {
        deps!();
        fn thin_to_thick < H , T > (thin : * mut ArcInner < HeaderSlice < H , [T ; 0] > > ,) -> * mut ArcInner < HeaderSlice < H , [T] > > { let len = unsafe { (* thin) . data . length } ; let fake_slice : * mut [T] = ptr :: slice_from_raw_parts_mut (thin as * mut T , len) ; fake_slice as * mut ArcInner < HeaderSlice < H , [T] > > }
    };
}

thin_to_thick!()