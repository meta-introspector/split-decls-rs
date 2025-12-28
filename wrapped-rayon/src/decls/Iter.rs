macro_rules! Iter {
    () => {
        # [doc = " Parallel iterator over immutable items in a slice"] # [derive (Debug)] pub struct Iter < 'data , T > { slice : & 'data [T] , }
    };
}

Iter!()