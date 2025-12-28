macro_rules! deps {
    () => {
        IntoParallelRefIterator!();
        IntoParallelIterator!();
        Iter!();
    };
}

macro_rules! impl_1026 {
    () => {
        deps!();
        impl < 'data , I : 'data + ? Sized > IntoParallelRefIterator < 'data > for I where & 'data I : IntoParallelIterator , { type Iter = < & 'data I as IntoParallelIterator > :: Iter ; type Item = < & 'data I as IntoParallelIterator > :: Item ; fn par_iter (& 'data self) -> Self :: Iter { self . into_par_iter () } }
    };
}

impl_1026!()