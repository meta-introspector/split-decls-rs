macro_rules! arbitrary_for_vec_vec_vec_u8 {
    () => {
        # [test] fn arbitrary_for_vec_vec_vec_u8 () { assert_generates :: < Vec < Vec < Vec < u8 > > > > ([vec ! [] , vec ! [vec ! []] , vec ! [vec ! [vec ! [0]]] , vec ! [vec ! [vec ! [1]]] , vec ! [vec ! [vec ! [0 , 1]]] , vec ! [vec ! [] , vec ! []] , vec ! [vec ! [] , vec ! [vec ! []]] , vec ! [vec ! [vec ! []] , vec ! []] , vec ! [vec ! [vec ! []] , vec ! [vec ! []]] , vec ! [vec ! [vec ! [0]] , vec ! []] , vec ! [vec ! [] , vec ! [vec ! [1]]] , vec ! [vec ! [vec ! [0]] , vec ! [vec ! [1]]] , vec ! [vec ! [vec ! [0 , 1]] , vec ! []] , vec ! [vec ! [] , vec ! [vec ! [0 , 1]]] , vec ! [vec ! [] , vec ! [] , vec ! []] , vec ! [vec ! [vec ! []] , vec ! [] , vec ! []] , vec ! [vec ! [] , vec ! [vec ! []] , vec ! []] , vec ! [vec ! [] , vec ! [] , vec ! [vec ! []]] ,]) ; }
    };
}

arbitrary_for_vec_vec_vec_u8!()