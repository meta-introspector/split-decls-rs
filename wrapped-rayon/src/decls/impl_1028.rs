macro_rules! deps {
    () => {
        Iter!();
        IntoParallelRefMutIterator!();
        IntoParallelIterator!();
    };
}

macro_rules! impl_1028 {
    () => {
        deps!();
        impl < 'data , I : 'data + ? Sized > IntoParallelRefMutIterator < 'data > for I where & 'data mut I : IntoParallelIterator , { type Iter = < & 'data mut I as IntoParallelIterator > :: Iter ; type Item = < & 'data mut I as IntoParallelIterator > :: Item ; fn par_iter_mut (& 'data mut self) -> Self :: Iter { self . into_par_iter () } }
    };
}

impl_1028!();