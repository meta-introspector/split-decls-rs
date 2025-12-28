macro_rules! deps {
    () => {
        Zip!();
    };
}

macro_rules! tests {
    () => {
        deps!();
        # [cfg (test)] mod tests { use alloc :: vec ; use crate :: stream :: Zip ; use futures_lite :: future :: block_on ; use futures_lite :: prelude :: * ; use futures_lite :: stream ; # [test] fn zip_array_3 () { block_on (async { let a = stream :: repeat (1) . take (2) ; let b = stream :: repeat (2) . take (2) ; let c = stream :: repeat (3) . take (2) ; let mut s = vec ! [a , b , c] . zip () ; assert_eq ! (s . next () . await , Some (vec ! [1 , 2 , 3])) ; assert_eq ! (s . next () . await , Some (vec ! [1 , 2 , 3])) ; assert_eq ! (s . next () . await , None) ; }) } }
    };
}

tests!()