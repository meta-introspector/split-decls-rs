macro_rules! vec_into_parts {
    () => {
        fn vec_into_parts < T > (vec : Vec < T >) -> (NonNull < T > , usize , usize) { let mut vec = ManuallyDrop :: new (vec) ; (unsafe { NonNull :: new_unchecked (vec . as_mut_ptr ()) } , vec . capacity () , vec . len () ,) }
    };
}

vec_into_parts!()