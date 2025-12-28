macro_rules! DOUBLE_ROUND {
    () => {
        macro_rules ! DOUBLE_ROUND { ($ r0 : expr , $ r1 : expr , $ r2 : expr , $ r3 : expr) => { ROUND ! ($ r0 , $ r1 , $ r2 , $ r3) ; $ r1 = $ r1 . shl_1 () ; $ r2 = $ r2 . shl_2 () ; $ r3 = $ r3 . shl_3 () ; ROUND ! ($ r0 , $ r1 , $ r2 , $ r3) ; $ r1 = $ r1 . shl_3 () ; $ r2 = $ r2 . shl_2 () ; $ r3 = $ r3 . shl_1 () ; } ; }
    };
}

DOUBLE_ROUND!();