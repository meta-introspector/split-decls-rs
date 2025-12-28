macro_rules! deps {
    () => {
        Table!();
        Id!();
    };
}

macro_rules! Item {
    () => {
        deps!();
        # [doc = " An item in a [`Table`]."] pub trait Item { # [doc = " The type of identifier for the item."] type Id : Id ; # [doc = " Return `True` if the item is deleted."] fn is_deleted (& self) -> bool ; }
    };
}

Item!()