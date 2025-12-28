macro_rules! deps {
    () => {
        TreeRef!();
        Tree!();
    };
}

macro_rules! impl_139 {
    () => {
        deps!();
        impl TreeRef < '_ > { # [doc = " Convert this instance into its own version, creating a copy of all data."] # [doc = ""] # [doc = " This will temporarily allocate an extra copy in memory, so at worst three copies of the tree exist"] # [doc = " at some intermediate point in time. Use [`Self::into_owned()`] to avoid this."] pub fn to_owned (& self) -> Tree { self . clone () . into () } # [doc = " Convert this instance into its own version, creating a copy of all data."] pub fn into_owned (self) -> Tree { self . into () } }
    };
}

impl_139!();