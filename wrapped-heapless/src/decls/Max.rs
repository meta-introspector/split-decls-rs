macro_rules! deps {
    () => {
        SortedLinkedList!();
    };
}

macro_rules! Max {
    () => {
        deps!();
        # [doc = " Marker for Max sorted [`SortedLinkedList`]."] pub struct Max ;
    };
}

Max!()