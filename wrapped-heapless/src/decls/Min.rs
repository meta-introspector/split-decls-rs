macro_rules! deps {
    () => {
        SortedLinkedList!();
    };
}

macro_rules! Min {
    () => {
        deps!();
        # [doc = " Marker for Min sorted [`SortedLinkedList`]."] pub struct Min ;
    };
}

Min!();