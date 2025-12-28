macro_rules! deps {
    () => {
        KeyType!();
    };
}

macro_rules! Key {
    () => {
        deps!();
        # [derive (PartialEq , Eq , Debug , Clone)] pub (crate) struct Key { key : KeyType , index : usize , }
    };
}

Key!();