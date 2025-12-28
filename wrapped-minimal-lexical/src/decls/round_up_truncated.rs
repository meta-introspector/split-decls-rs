macro_rules! round_up_truncated {
    () => {
        # [doc = " Round-up a truncated value."] macro_rules ! round_up_truncated { ($ format : ident , $ result : ident , $ count : ident) => { { add_temporary ! (@ mul $ result , 10 , 1) ; $ count += 1 ; } } ; }
    };
}

round_up_truncated!();