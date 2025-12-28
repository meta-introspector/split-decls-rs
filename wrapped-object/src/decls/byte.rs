macro_rules! deps {
    () => {
        IndexSet!();
    };
}

macro_rules! byte {
    () => {
        deps!();
        fn byte (id : usize , pos : usize , strings : & IndexSet < & [u8] >) -> u8 { let string = strings . get_index (id) . unwrap () ; let len = string . len () ; if len >= pos { string [len - pos] } else { 0 } }
    };
}

byte!();