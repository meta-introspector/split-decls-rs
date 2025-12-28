macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl < 'a , IDX , V > Entry < 'a , IDX , V > where V : Default , { # [doc = " Ensures a value is in the entry by inserting the default value if empty, and returns a mutable reference"] # [doc = " to the value in the entry."] # [allow (clippy :: unwrap_or_default)] pub fn or_default (self) -> & 'a mut V { self . or_insert_with (Default :: default) } }
    };
}

impl_14!()