macro_rules! deps {
    () => {
        SortedLinkedListInner!();
    };
}

macro_rules! SortedLinkedListView {
    () => {
        deps!();
        # [doc = " The linked list."] pub type SortedLinkedListView < T , K , Idx > = SortedLinkedListInner < T , Idx , K , ViewSortedLinkedListStorage < T , Idx > > ;
    };
}

SortedLinkedListView!();