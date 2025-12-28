macro_rules! deps {
    () => {
        SortedLinkedList!();
        SortedLinkedListView!();
        LenType!();
    };
}

macro_rules! SortedLinkedListInner {
    () => {
        deps!();
        # [doc = " Base struct for [`SortedLinkedList`] and [`SortedLinkedListView`], generic over the"] # [doc = " [`SortedLinkedListStorage`]."] # [doc = ""] # [doc = " In most cases you should use [`SortedLinkedList`] or [`SortedLinkedListView`] directly. Only use"] # [doc = " this struct if you want to write code that's generic over both."] pub struct SortedLinkedListInner < T , Idx , K , S > where Idx : LenType , S : SortedLinkedListStorage < T , Idx > + ? Sized , { head : Idx , free : Idx , phantom : PhantomData < (K , T) > , list : S , }
    };
}

SortedLinkedListInner!()