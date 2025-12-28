macro_rules! deps {
    () => {
        ThreadPoolBuilder!();
    };
}

macro_rules! test_nested_order {
    () => {
        deps!();
        macro_rules ! test_nested_order { ($ outer_scope : ident => $ outer_spawn : ident , $ inner_scope : ident => $ inner_spawn : ident) => { { let builder = ThreadPoolBuilder :: new () . num_threads (1) ; let pool = builder . build () . unwrap () ; pool . install (|| { let vec = Mutex :: new (vec ! []) ; $ outer_scope (| scope | { let vec = & vec ; for i in 0 .. 10 { scope .$ outer_spawn (move | _ | { $ inner_scope (| scope | { for j in 0 .. 10 { scope .$ inner_spawn (move | _ | { vec . lock () . unwrap () . push (i * 10 + j) ; }) ; } }) ; }) ; } }) ; vec . into_inner () . unwrap () }) } } ; }
    };
}

test_nested_order!();