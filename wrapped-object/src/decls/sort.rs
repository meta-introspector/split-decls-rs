macro_rules! deps {
    () => {
        IndexSet!();
    };
}

macro_rules! sort {
    () => {
        deps!();
        fn sort (mut ids : & mut [usize] , mut pos : usize , strings : & IndexSet < & [u8] >) { loop { if ids . len () <= 1 { return ; } let pivot = byte (ids [0] , pos , strings) ; let mut lower = 0 ; let mut upper = ids . len () ; let mut i = 1 ; while i < upper { let b = byte (ids [i] , pos , strings) ; if b > pivot { ids . swap (lower , i) ; lower += 1 ; i += 1 ; } else if b < pivot { upper -= 1 ; ids . swap (upper , i) ; } else { i += 1 ; } } sort (& mut ids [.. lower] , pos , strings) ; sort (& mut ids [upper ..] , pos , strings) ; if pivot == 0 { return ; } ids = & mut ids [lower .. upper] ; pos += 1 ; } }
    };
}

sort!()