macro_rules! test_keys_affect_every_byte {
    () => {
        # [doc = "Ensures that for every bit in the output there is some value for each byte in the key that flips it."] fn test_keys_affect_every_byte < H : Hash , T : Hasher > (item : H , constructor : impl Fn (u128 , u128) -> T) { let base = hash_with (& item , constructor (0 , 0)) ; for shift in 0 .. 16 { let mut alternatives1 = vec ! [] ; let mut alternatives2 = vec ! [] ; for v in 0 .. 256 { let input = (v as u128) << (shift * 8) ; let hasher1 = constructor (input , 0) ; let hasher2 = constructor (0 , input) ; let h1 = hash_with (& item , hasher1) ; let h2 = hash_with (& item , hasher2) ; alternatives1 . push (h1) ; alternatives2 . push (h2) ; } assert_each_byte_differs (shift , base , alternatives1) ; assert_each_byte_differs (shift , base , alternatives2) ; } }
    };
}

test_keys_affect_every_byte!();