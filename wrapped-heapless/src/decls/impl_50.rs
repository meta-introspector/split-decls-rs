macro_rules! deps {
    () => {
        Deque!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl < T : PartialEq , const N : usize > PartialEq for Deque < T , N > { fn eq (& self , other : & Self) -> bool { if self . len () != other . len () { return false ; } let (sa , sb) = self . as_slices () ; let (oa , ob) = other . as_slices () ; match sa . len () . cmp (& oa . len ()) { Ordering :: Equal => sa == oa && sb == ob , Ordering :: Less => { let front = sa . len () ; let mid = oa . len () - front ; let (oa_front , oa_mid) = oa . split_at (front) ; let (sb_mid , sb_back) = sb . split_at (mid) ; debug_assert_eq ! (sa . len () , oa_front . len ()) ; debug_assert_eq ! (sb_mid . len () , oa_mid . len ()) ; debug_assert_eq ! (sb_back . len () , ob . len ()) ; sa == oa_front && sb_mid == oa_mid && sb_back == ob } Ordering :: Greater => { let front = oa . len () ; let mid = sa . len () - front ; let (sa_front , sa_mid) = sa . split_at (front) ; let (ob_mid , ob_back) = ob . split_at (mid) ; debug_assert_eq ! (sa_front . len () , oa . len ()) ; debug_assert_eq ! (sa_mid . len () , ob_mid . len ()) ; debug_assert_eq ! (sb . len () , ob_back . len ()) ; sa_front == oa && sa_mid == ob_mid && sb == ob_back } } } }
    };
}

impl_50!();