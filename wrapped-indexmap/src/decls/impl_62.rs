macro_rules! deps {
    () => {
        IndexMap!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl < K , V , S > Extend < (K , V) > for IndexMap < K , V , S > where K : Hash + Eq , S : BuildHasher , { # [doc = " Extend the map with all key-value pairs in the iterable."] # [doc = ""] # [doc = " This is equivalent to calling [`insert`][IndexMap::insert] for each of"] # [doc = " them in order, which means that for keys that already existed"] # [doc = " in the map, their value is updated but it keeps the existing order."] # [doc = ""] # [doc = " New keys are inserted in the order they appear in the sequence. If"] # [doc = " equivalents of a key occur more than once, the last corresponding value"] # [doc = " prevails."] fn extend < I : IntoIterator < Item = (K , V) > > (& mut self , iterable : I) { let iter = iterable . into_iter () ; let (lower_len , _) = iter . size_hint () ; let reserve = if self . is_empty () { lower_len } else { lower_len . div_ceil (2) } ; self . reserve (reserve) ; iter . for_each (move | (k , v) | { self . insert (k , v) ; }) ; } }
    };
}

impl_62!();