macro_rules! LENGTH_EXTRA {
    () => {
        # [doc = " Number of extra bits for each length code."] # [rustfmt :: skip] const LENGTH_EXTRA : [u8 ; 32] = [0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 1 , 1 , 1 , 1 , 2 , 2 , 2 , 2 , 3 , 3 , 3 , 3 , 4 , 4 , 4 , 4 , 5 , 5 , 5 , 5 , 0 , 0 , 0 , 0] ;
    };
}

LENGTH_EXTRA!()