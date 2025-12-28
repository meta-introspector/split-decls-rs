macro_rules! deps {
    () => {
        Encoding!();
        ListsHeader!();
        Format!();
    };
}

macro_rules! impl_437 {
    () => {
        deps!();
        impl Default for ListsHeader { fn default () -> Self { ListsHeader { encoding : Encoding { format : Format :: Dwarf32 , version : 5 , address_size : 0 , } , offset_entry_count : 0 , } } }
    };
}

impl_437!();