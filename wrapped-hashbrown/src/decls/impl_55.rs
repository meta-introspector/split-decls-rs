macro_rules! deps {
    () => {
        RawTableInner!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl RawTableInner { const NEW : Self = RawTableInner :: new () ; # [doc = " Creates a new empty hash table without allocating any memory."] # [doc = ""] # [doc = " In effect this returns a table with exactly 1 bucket. However we can"] # [doc = " leave the data pointer dangling since that bucket is never accessed"] # [doc = " due to our load factor forcing us to always have at least 1 free bucket."] # [inline] const fn new () -> Self { Self { ctrl : unsafe { NonNull :: new_unchecked (Group :: static_empty () . as_ptr () . cast_mut () . cast ()) } , bucket_mask : 0 , items : 0 , growth_left : 0 , } } }
    };
}

impl_55!()