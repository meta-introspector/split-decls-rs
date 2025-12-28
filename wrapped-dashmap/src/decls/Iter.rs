macro_rules! Iter {
    () => {
        pub struct Iter < 'a , K > { inner : crate :: iter :: Iter < 'a , K , () > , }
    };
}

Iter!();