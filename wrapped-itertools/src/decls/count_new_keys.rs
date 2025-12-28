macro_rules! count_new_keys {
    () => {
        fn count_new_keys < I , K > (mut used : HashMap < K , () > , iterable : I) -> usize where I : IntoIterator < Item = K > , K : Hash + Eq , { let iter = iterable . into_iter () ; let current_used = used . len () ; used . extend (iter . map (| key | (key , ()))) ; used . len () - current_used }
    };
}

count_new_keys!();