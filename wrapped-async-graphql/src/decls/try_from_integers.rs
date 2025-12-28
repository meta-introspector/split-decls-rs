macro_rules! deps {
    () => {
        Error!();
        ID!();
        Result!();
    };
}

macro_rules! try_from_integers {
    () => {
        deps!();
        macro_rules ! try_from_integers { ($ ($ ty : ty) ,*) => { $ (impl TryFrom < ID > for $ ty { type Error = ParseIntError ; fn try_from (id : ID) -> Result < Self , Self :: Error > { id . 0 . parse () } }) * } ; }
    };
}

try_from_integers!();