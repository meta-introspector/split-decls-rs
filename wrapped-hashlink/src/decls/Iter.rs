macro_rules! deps {
    () => {
        Keys!();
    };
}

macro_rules! Iter {
    () => {
        deps!();
        pub struct Iter < 'a , K > { iter : linked_hash_map :: Keys < 'a , K , () > , }
    };
}

Iter!();