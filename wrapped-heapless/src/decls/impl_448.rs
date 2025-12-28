macro_rules! deps {
    () => {
        Kind!();
        SortedLinkedListInner!();
        LenType!();
    };
}

macro_rules! impl_448 {
    () => {
        deps!();
        # [cfg (feature = "zeroize")] impl < T , Idx , K , S > Zeroize for SortedLinkedListInner < T , Idx , K , S > where T : Ord + Zeroize , Idx : LenType + Zeroize , K : Kind , S : SortedLinkedListStorage < T , Idx > + ? Sized , { fn zeroize (& mut self) { while let Some (mut item) = self . pop () { item . zeroize () ; } let buffer = self . list . borrow_mut () ; for elem in buffer { elem . zeroize () ; } } }
    };
}

impl_448!()