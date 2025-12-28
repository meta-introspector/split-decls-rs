macro_rules! deps {
    () => {
        Deserializer!();
    };
}

macro_rules! SeqAccess {
    () => {
        deps!();
        struct SeqAccess < 'a , R : 'a > { de : & 'a mut Deserializer < R > , first : bool , }
    };
}

SeqAccess!();