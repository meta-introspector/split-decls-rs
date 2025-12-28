macro_rules! ROUND {
    () => {
        macro_rules ! ROUND { ($ r0 : expr , $ r1 : expr , $ r2 : expr , $ r3 : expr) => { $ r0 = $ r0 . wrapping_add ($ r1) ; $ r3 = ($ r3 ^ $ r0) . rotate_left (16) ; $ r2 = $ r2 . wrapping_add ($ r3) ; $ r1 = ($ r1 ^ $ r2) . rotate_left (12) ; $ r0 = $ r0 . wrapping_add ($ r1) ; $ r3 = ($ r3 ^ $ r0) . rotate_left (8) ; $ r2 = $ r2 . wrapping_add ($ r3) ; $ r1 = ($ r1 ^ $ r2) . rotate_left (7) ; } ; }
    };
}

ROUND!()