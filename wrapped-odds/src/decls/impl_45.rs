macro_rules! deps {
    () => {
        SliceCopyIter!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl < 'a , T > From < & 'a [T] > for SliceCopyIter < 'a , T > where T : Copy , { fn from (slice : & 'a [T]) -> Self { assert ! (size_of ::< T > () != 0) ; unsafe { let ptr = slice . as_ptr () ; let end = ptr . offset (slice . len () as isize) ; SliceCopyIter :: new (ptr , end) } } }
    };
}

impl_45!();