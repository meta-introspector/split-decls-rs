macro_rules! deps {
    () => {
        Encoding!();
    };
}

macro_rules! ListsHeader {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] pub (crate) struct ListsHeader { encoding : Encoding , # [allow (dead_code)] offset_entry_count : u32 , }
    };
}

ListsHeader!()