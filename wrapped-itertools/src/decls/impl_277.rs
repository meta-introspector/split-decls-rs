macro_rules! deps {
    () => {
        IntoChunks!();
    };
}

macro_rules! impl_277 {
    () => {
        deps!();
        impl < I > IntoChunks < I > where I : Iterator , { # [doc = " `client`: Index of chunk that requests next element"] fn step (& self , client : usize) -> Option < I :: Item > { self . inner . borrow_mut () . step (client) } # [doc = " `client`: Index of chunk"] fn drop_group (& self , client : usize) { self . inner . borrow_mut () . drop_group (client) ; } }
    };
}

impl_277!();