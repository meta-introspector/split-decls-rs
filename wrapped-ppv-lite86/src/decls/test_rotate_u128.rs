macro_rules! test_rotate_u128 {
    () => {
        # [test] fn test_rotate_u128 () { const X : u128 = 0x0001_0203_0405_0607_0809_0a0b_0c0d_0e0f ; assert_eq ! (rotate_u128_right (X , 17) , X . rotate_right (17)) ; }
    };
}

test_rotate_u128!()