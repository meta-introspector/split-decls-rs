macro_rules! deps {
    () => {
        CLruCache!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl < K : Clone + Eq + Hash , V , S : BuildHasher + Default > FromIterator < (K , V) > for CLruCache < K , V , S > { fn from_iter < I : IntoIterator < Item = (K , V) > > (iter : I) -> Self { let cap = NonZeroUsize :: new (usize :: MAX) . unwrap () ; let mut cache = CLruCache :: with_hasher (cap , S :: default ()) ; for (k , v) in iter { cache . put (k , v) ; } cache . resize (NonZeroUsize :: new (cache . len ()) . unwrap_or_else (| | NonZeroUsize :: new (1) . unwrap ()) ,) ; cache } }
    };
}

impl_27!()