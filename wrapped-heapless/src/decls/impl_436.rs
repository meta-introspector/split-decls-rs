macro_rules! deps {
    () => {
        Vec!();
        Node!();
        SortedLinkedList!();
        LenType!();
        SortedLinkedListInner!();
        SortedLinkedListView!();
    };
}

macro_rules! impl_436 {
    () => {
        deps!();
        impl < T , Idx , K , S > SortedLinkedListInner < T , Idx , K , S > where Idx : LenType , S : SortedLinkedListStorage < T , Idx > + ? Sized , { # [doc = " Get a reference to the `SortedLinkedList`, erasing the `N` const-generic."] pub fn as_view (& self) -> & SortedLinkedListView < T , K , Idx > { S :: as_view (self) } # [doc = " Get a mutable reference to the `Vec`, erasing the `N` const-generic."] pub fn as_mut_view (& mut self) -> & mut SortedLinkedListView < T , K , Idx > { S :: as_mut_view (self) } # [doc = " Internal access helper"] # [inline (always)] fn node_at (& self , index : usize) -> & Node < T , Idx > { unsafe { self . list . borrow () . get_unchecked (index) } } # [doc = " Internal access helper"] # [inline (always)] fn node_at_mut (& mut self , index : usize) -> & mut Node < T , Idx > { unsafe { self . list . borrow_mut () . get_unchecked_mut (index) } } # [doc = " Internal access helper"] # [inline (always)] fn write_data_in_node_at (& mut self , index : usize , data : T) { unsafe { self . node_at_mut (index) . val . as_mut_ptr () . write (data) ; } } # [doc = " Internal access helper"] # [inline (always)] fn read_data_in_node_at (& self , index : usize) -> & T { unsafe { & * self . node_at (index) . val . as_ptr () } } # [doc = " Internal access helper"] # [inline (always)] fn read_mut_data_in_node_at (& mut self , index : usize) -> & mut T { unsafe { & mut * self . node_at_mut (index) . val . as_mut_ptr () } } # [doc = " Internal access helper"] # [inline (always)] fn extract_data_in_node_at (& mut self , index : usize) -> T { unsafe { self . node_at (index) . val . as_ptr () . read () } } }
    };
}

impl_436!();