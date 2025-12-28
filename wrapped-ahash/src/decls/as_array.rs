macro_rules! as_array {
    () => {
        macro_rules ! as_array { ($ input : expr , $ len : expr) => { { { # [inline (always)] fn as_array < T > (slice : & [T]) -> & [T ; $ len] { core :: convert :: TryFrom :: try_from (slice) . unwrap () } as_array ($ input) } } } ; }
    };
}

as_array!()