macro_rules! deps {
    () => {
        RepeatN!();
        Iter!();
        IntoParallelIterator!();
        Repeat!();
        IndexedParallelIterator!();
        Zip!();
    };
}

macro_rules! impl_797 {
    () => {
        deps!();
        impl < T > Repeat < T > where T : Clone + Send , { # [doc = " Takes only `n` repeats of the element, similar to the general"] # [doc = " [`take()`]."] # [doc = ""] # [doc = " The resulting `RepeatN` is an `IndexedParallelIterator`, allowing"] # [doc = " more functionality than `Repeat` alone."] # [doc = ""] # [doc = " [`take()`]: IndexedParallelIterator::take()"] pub fn take (self , n : usize) -> RepeatN < T > { repeat_n (self . element , n) } # [doc = " Iterates tuples, repeating the element with items from another"] # [doc = " iterator, similar to the general [`zip()`]."] # [doc = ""] # [doc = " [`zip()`]: IndexedParallelIterator::zip()"] pub fn zip < Z > (self , zip_op : Z) -> Zip < RepeatN < T > , Z :: Iter > where Z : IntoParallelIterator < Iter : IndexedParallelIterator > , { let z = zip_op . into_par_iter () ; let n = z . len () ; self . take (n) . zip (z) } }
    };
}

impl_797!();