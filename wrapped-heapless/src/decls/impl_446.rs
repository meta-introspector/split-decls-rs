macro_rules! deps {
    () => {
        LenType!();
        SortedLinkedListInner!();
        Kind!();
    };
}

macro_rules! impl_446 {
    () => {
        deps!();
        impl < T , Idx , K , S > fmt :: Debug for SortedLinkedListInner < T , Idx , K , S > where T : Ord + core :: fmt :: Debug , Idx : LenType , K : Kind , S : ? Sized + SortedLinkedListStorage < T , Idx > , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . iter ()) . finish () } }
    };
}

impl_446!();