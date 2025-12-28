macro_rules! deps {
    () => {
        HashSet!();
    };
}

macro_rules! impl_198 {
    () => {
        deps!();
        # [doc = " Collect values from a parallel iterator into a hashset."] impl < T , S > FromParallelIterator < T > for HashSet < T , S , Global > where T : Eq + Hash + Send , S : BuildHasher + Default , { fn from_par_iter < P > (par_iter : P) -> Self where P : IntoParallelIterator < Item = T > , { let mut set = HashSet :: default () ; set . par_extend (par_iter) ; set } }
    };
}

impl_198!();