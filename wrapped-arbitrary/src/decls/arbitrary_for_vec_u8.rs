macro_rules! arbitrary_for_vec_u8 {
    () => {
        # [test] fn arbitrary_for_vec_u8 () { assert_generates :: < Vec < u8 > > ([vec ! [] , vec ! [0] , vec ! [1] , vec ! [0 , 0] , vec ! [0 , 1] , vec ! [1 , 0] , vec ! [1 , 1] , vec ! [0 , 0 , 0] , vec ! [0 , 0 , 1] , vec ! [0 , 1 , 0] , vec ! [0 , 1 , 1] , vec ! [1 , 0 , 0] , vec ! [1 , 0 , 1] , vec ! [1 , 1 , 0] , vec ! [1 , 1 , 1] ,]) ; }
    };
}

arbitrary_for_vec_u8!();