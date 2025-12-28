macro_rules! deps {
    () => {
        ListsHeader!();
    };
}

macro_rules! RngListsHeader {
    () => {
        deps!();
        # [allow (unused)] pub (crate) type RngListsHeader = ListsHeader ;
    };
}

RngListsHeader!()