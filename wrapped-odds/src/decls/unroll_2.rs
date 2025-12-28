macro_rules! unroll_2 {
    () => {
        # [cfg (test)] pub fn unroll_2 < 'a , T , F > (data : & 'a [T] , mut f : F) where F : FnMut (& 'a T) , { let mut data = data ; while data . len () >= 2 { f (& data [0]) ; f (& data [1]) ; data = & data [2 ..] ; } if 0 < data . len () { f (& data [0]) ; } }
    };
}

unroll_2!();