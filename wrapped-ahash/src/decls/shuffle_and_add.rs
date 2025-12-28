macro_rules! shuffle_and_add {
    () => {
        # [allow (unused)] # [inline (always)] pub (crate) fn shuffle_and_add (base : u128 , to_add : u128) -> u128 { let shuffled : [u64 ; 2] = shuffle (base) . convert () ; add_by_64s (shuffled , to_add . convert ()) . convert () }
    };
}

shuffle_and_add!()