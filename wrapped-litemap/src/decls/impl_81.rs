macro_rules! deps {
    () => {
        StoreBulkMut!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        impl < K : Ord , V > StoreBulkMut < K , V > for Vec < (K , V) > { # [inline] fn lm_retain < F > (& mut self , mut predicate : F) where F : FnMut (& K , & V) -> bool , { self . retain (| (k , v) | predicate (k , v)) } # [doc = " Extends this store with items from an iterator."] # [doc = ""] # [doc = " It uses a two-pass (sort + dedup) approach to avoid any potential quadratic costs."] # [doc = ""] # [doc = " The asymptotic worst case complexity is O((n + m) log(n + m)), where `n`"] # [doc = " is the number of elements already in `self` and `m` is the number of elements"] # [doc = " in the iterator. The best case complexity is O(m), when the input iterator is"] # [doc = " already sorted, keys aren't duplicated and all keys sort after the existing ones."] # [inline] fn lm_extend < I > (& mut self , iter : I) where I : IntoIterator < Item = (K , V) > , K : Ord , { let mut sorted_len = self . len () ; self . extend (iter) ; # [expect (clippy :: indexing_slicing)] { sorted_len += self [sorted_len . saturating_sub (1) ..] . windows (2) . take_while (| w | w [0] . 0 < w [1] . 0) . count () ; } sorted_len += (sorted_len == 0 && ! self . is_empty ()) as usize ; if sorted_len >= self . len () { return ; } self . sort_by (| a , b | a . 0 . cmp (& b . 0)) ; let (dedup , _merged_dup) = partition_dedup_by (self) ; sorted_len = dedup . len () ; self . truncate (sorted_len) ; } }
    };
}

impl_81!()