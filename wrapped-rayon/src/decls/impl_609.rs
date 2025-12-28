macro_rules! deps {
    () => {
        FromParallelIterator!();
        IntoParallelIterator!();
    };
}

macro_rules! impl_609 {
    () => {
        deps!();
        # [doc = " Collects an arbitrary `Cow` collection."] # [doc = ""] # [doc = " Note, the standard library only has `FromIterator` for `Cow<'a, str>` and"] # [doc = " `Cow<'a, [T]>`, because no one thought to add a blanket implementation"] # [doc = " before it was stabilized."] impl < 'a , C , T > FromParallelIterator < T > for Cow < 'a , C > where C : ToOwned < Owned : FromParallelIterator < T > > + ? Sized , T : Send , { fn from_par_iter < I > (par_iter : I) -> Self where I : IntoParallelIterator < Item = T > , { Cow :: Owned (C :: Owned :: from_par_iter (par_iter)) } }
    };
}

impl_609!()