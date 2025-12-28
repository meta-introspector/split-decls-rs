macro_rules! deps {
    () => {
        Kind!();
        SortedLinkedListView!();
        LenType!();
    };
}

macro_rules! FindMutView {
    () => {
        deps!();
        # [doc = " Comes from [`SortedLinkedList::find_mut`]."] pub struct FindMutView < 'a , T , Idx , K > where T : Ord , Idx : LenType , K : Kind , { list : & 'a mut SortedLinkedListView < T , K , Idx > , is_head : bool , prev_index : Idx , index : Idx , maybe_changed : bool , }
    };
}

FindMutView!()