macro_rules! deps {
    () => {
        PotentialCodePoint!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl TryFrom < PotentialCodePoint > for char { type Error = core :: char :: CharTryFromError ; # [inline] fn try_from (value : PotentialCodePoint) -> Result < char , Self :: Error > { value . try_to_char () } }
    };
}

impl_15!();