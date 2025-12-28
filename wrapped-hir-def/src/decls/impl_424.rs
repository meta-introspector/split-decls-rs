macro_rules! deps {
    () => {
        ImportMap!();
    };
}

macro_rules! impl_424 {
    () => {
        deps!();
        impl PartialEq for ImportMap { fn eq (& self , other : & Self) -> bool { self . item_to_info_map == other . item_to_info_map } }
    };
}

impl_424!();