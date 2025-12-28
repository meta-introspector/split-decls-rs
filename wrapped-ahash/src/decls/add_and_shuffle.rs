macro_rules! add_and_shuffle {
    () => {
        # [allow (unused)] # [inline (always)] pub (crate) fn add_and_shuffle (a : u128 , b : u128) -> u128 { let sum = add_by_64s (a . convert () , b . convert ()) ; shuffle (sum . convert ()) }
    };
}

add_and_shuffle!()