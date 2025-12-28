macro_rules! deps {
    () => {
        GeCached!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        impl GeCached { pub fn maybe_set (& mut self , other : & GeCached , do_swap : u8) { self . y_plus_x . maybe_set (& other . y_plus_x , do_swap) ; self . y_minus_x . maybe_set (& other . y_minus_x , do_swap) ; self . z . maybe_set (& other . z , do_swap) ; self . t2d . maybe_set (& other . t2d , do_swap) ; } }
    };
}

impl_103!()