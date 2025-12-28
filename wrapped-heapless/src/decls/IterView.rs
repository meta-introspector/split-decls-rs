macro_rules! deps {
    () => {
        LenType!();
        Kind!();
        SortedLinkedListInner!();
    };
}

macro_rules! IterView {
    () => {
        deps!();
        # [doc = " Iterator for the linked list."] pub struct IterView < 'a , T , Idx , K > where T : Ord , Idx : LenType , K : Kind , { list : & 'a SortedLinkedListInner < T , Idx , K , ViewSortedLinkedListStorage < T , Idx > > , index : Idx , }
    };
}

IterView!()