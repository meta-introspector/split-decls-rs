macro_rules! i {
    () => {
        # [doc = " # Safety"] # [doc = ""] # [doc = " Safe if `index < array.len()`."] macro_rules ! i { ($ array : ident , $ index : expr) => { unsafe { *$ array . get_unchecked ($ index) } } ; }
    };
}

i!()