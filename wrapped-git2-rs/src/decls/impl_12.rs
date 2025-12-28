macro_rules! impl_12 {
    () => {
        impl Sort { is_bit_set ! (is_none , Sort :: NONE) ; is_bit_set ! (is_topological , Sort :: TOPOLOGICAL) ; is_bit_set ! (is_time , Sort :: TIME) ; is_bit_set ! (is_reverse , Sort :: REVERSE) ; }
    };
}

impl_12!()