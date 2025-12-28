macro_rules! Myers {
    () => {
        pub struct Myers { kvec : NonNull < [i32] > , kforward : NonNull < i32 > , kbackward : NonNull < i32 > , max_cost : u32 , }
    };
}

Myers!();