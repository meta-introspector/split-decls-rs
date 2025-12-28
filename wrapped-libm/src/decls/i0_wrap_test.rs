macro_rules! i0_wrap_test {
    () => {
        # [test] fn i0_wrap_test () { let x = - 3.0 / 256.0 ; assert_eq ! (exp2 (x) , f64 :: from_bits (0x3fefbdba3692d514)) ; }
    };
}

i0_wrap_test!();