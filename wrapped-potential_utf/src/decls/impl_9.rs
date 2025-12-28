macro_rules! deps {
    () => {
        PotentialCodePoint!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl PartialEq < PotentialCodePoint > for char { fn eq (& self , other : & PotentialCodePoint) -> bool { PotentialCodePoint :: from_char (* self) . eq (other) } }
    };
}

impl_9!();