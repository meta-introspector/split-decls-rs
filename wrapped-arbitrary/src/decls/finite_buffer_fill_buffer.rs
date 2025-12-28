macro_rules! deps {
    () => {
        Unstructured!();
    };
}

macro_rules! finite_buffer_fill_buffer {
    () => {
        deps!();
        # [test] fn finite_buffer_fill_buffer () { let x = [1 , 2 , 3 , 4] ; let mut rb = Unstructured :: new (& x) ; let mut z = [0 ; 2] ; rb . fill_buffer (& mut z) . unwrap () ; assert_eq ! (z , [1 , 2]) ; rb . fill_buffer (& mut z) . unwrap () ; assert_eq ! (z , [3 , 4]) ; rb . fill_buffer (& mut z) . unwrap () ; assert_eq ! (z , [0 , 0]) ; }
    };
}

finite_buffer_fill_buffer!()