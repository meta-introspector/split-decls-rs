macro_rules! SplitMut {
    () => {
        # [doc = " Parallel iterator over mutable slices separated by a predicate"] pub struct SplitMut < 'data , T , P > { slice : & 'data mut [T] , separator : P , }
    };
}

SplitMut!();