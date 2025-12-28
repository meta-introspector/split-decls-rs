macro_rules! deps {
    () => {
        ListsHeader!();
    };
}

macro_rules! LocListsHeader {
    () => {
        deps!();
        pub (crate) type LocListsHeader = ListsHeader ;
    };
}

LocListsHeader!();