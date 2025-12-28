macro_rules! deps {
    () => {
        Kind!();
        LenType!();
        SortedLinkedList!();
        SortedLinkedListInner!();
        Max!();
        IterView!();
        FindMutView!();
    };
}

macro_rules! impl_438 {
    () => {
        deps!();
        impl < T , Idx , K , S > SortedLinkedListInner < T , Idx , K , S > where T : Ord , Idx : LenType , K : Kind , S : SortedLinkedListStorage < T , Idx > + ? Sized , { # [doc = " Get an iterator over the sorted list."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use heapless::sorted_linked_list::{Max, SortedLinkedList};"] # [doc = " let mut ll: SortedLinkedList<_, Max, 3, u8> = SortedLinkedList::new_u8();"] # [doc = ""] # [doc = " ll.push(1).unwrap();"] # [doc = " ll.push(2).unwrap();"] # [doc = ""] # [doc = " let mut iter = ll.iter();"] # [doc = ""] # [doc = " assert_eq!(iter.next(), Some(&2));"] # [doc = " assert_eq!(iter.next(), Some(&1));"] # [doc = " assert_eq!(iter.next(), None);"] # [doc = " ```"] pub fn iter (& self) -> IterView < '_ , T , Idx , K > { IterView { list : S :: as_view (self) , index : self . head , } } # [doc = " Find an element in the list that can be changed and resorted."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use heapless::sorted_linked_list::{Max, SortedLinkedList};"] # [doc = " let mut ll: SortedLinkedList<_, Max, 3, u8> = SortedLinkedList::new_u8();"] # [doc = ""] # [doc = " ll.push(1).unwrap();"] # [doc = " ll.push(2).unwrap();"] # [doc = " ll.push(3).unwrap();"] # [doc = ""] # [doc = " // Find a value and update it"] # [doc = " let mut find = ll.find_mut(|v| *v == 2).unwrap();"] # [doc = " *find += 1000;"] # [doc = " find.finish();"] # [doc = ""] # [doc = " assert_eq!(ll.pop(), Some(1002));"] # [doc = " assert_eq!(ll.pop(), Some(3));"] # [doc = " assert_eq!(ll.pop(), Some(1));"] # [doc = " assert_eq!(ll.pop(), None);"] # [doc = " ```"] pub fn find_mut < F > (& mut self , mut f : F) -> Option < FindMutView < '_ , T , Idx , K > > where F : FnMut (& T) -> bool , { let head = self . head . to_non_max () ? ; if f (self . read_data_in_node_at (head)) { return Some (FindMutView { is_head : true , prev_index : Idx :: MAX , index : self . head , list : S :: as_mut_view (self) , maybe_changed : false , }) ; } let mut current = head ; while let Some (next) = self . node_at (current) . next . to_non_max () { if f (self . read_data_in_node_at (next)) { return Some (FindMutView { is_head : false , prev_index : Idx :: from_usize (current) , index : Idx :: from_usize (next) , list : S :: as_mut_view (self) , maybe_changed : false , }) ; } current = next ; } None } }
    };
}

impl_438!()