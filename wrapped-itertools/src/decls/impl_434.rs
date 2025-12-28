macro_rules! deps {
    () => {
        PermutationState!();
        SizeHint!();
    };
}

macro_rules! impl_434 {
    () => {
        deps!();
        impl PermutationState { fn size_hint_for (& self , n : usize) -> SizeHint { let at_start = | n , k | { debug_assert ! (n >= k) ; let total = (n - k + 1 ..= n) . try_fold (1usize , | acc , i | acc . checked_mul (i)) ; (total . unwrap_or (usize :: MAX) , total) } ; match * self { Self :: Start { k } if n < k => (0 , Some (0)) , Self :: Start { k } => at_start (n , k) , Self :: Buffered { k , min_n } => { size_hint :: sub_scalar (at_start (n , k) , min_n - k + 1) } Self :: Loaded { ref indices , ref cycles , } => { let count = cycles . iter () . enumerate () . try_fold (0usize , | acc , (i , & c) | { acc . checked_mul (indices . len () - i) . and_then (| count | count . checked_add (c)) }) ; (count . unwrap_or (usize :: MAX) , count) } Self :: End => (0 , Some (0)) , } } }
    };
}

impl_434!();