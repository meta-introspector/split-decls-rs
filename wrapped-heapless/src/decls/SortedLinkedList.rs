macro_rules! deps {
    () => {
        SortedLinkedListInner!();
    };
}

macro_rules! SortedLinkedList {
    () => {
        deps!();
        # [doc = " The linked list."] pub type SortedLinkedList < T , K , const N : usize , Idx = usize > = SortedLinkedListInner < T , Idx , K , OwnedSortedLinkedListStorage < T , Idx , N > > ;
    };
}

SortedLinkedList!()