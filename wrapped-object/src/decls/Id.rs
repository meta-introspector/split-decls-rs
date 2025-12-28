macro_rules! deps {
    () => {
        Table!();
    };
}

macro_rules! Id {
    () => {
        deps!();
        # [doc = " An identifier for referring to an item in a [`Table`]."] pub trait Id : IdPrivate { # [doc = " Return the index of the item in the table."] fn index (& self) -> usize ; }
    };
}

Id!();