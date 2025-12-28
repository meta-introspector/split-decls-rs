macro_rules! IterMut {
    () => {
        # [doc = " Parallel iterator over mutable items in a slice"] # [derive (Debug)] pub struct IterMut < 'data , T > { slice : & 'data mut [T] , }
    };
}

IterMut!()