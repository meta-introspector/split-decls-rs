macro_rules! deps {
    () => {
        LenType!();
        Max!();
        FindMutView!();
        Kind!();
        SortedLinkedList!();
    };
}

macro_rules! impl_442 {
    () => {
        deps!();
        impl < T , Idx , K > FindMutView < '_ , T , Idx , K > where T : Ord , Idx : LenType , K : Kind , { fn pop_internal (& mut self) -> T { if self . is_head { unsafe { self . list . pop_unchecked () } } else { let prev = self . prev_index . into_usize () ; let curr = self . index . into_usize () ; self . list . node_at_mut (prev) . next = self . list . node_at_mut (curr) . next ; self . list . node_at_mut (curr) . next = self . list . free ; self . list . free = self . index ; self . list . extract_data_in_node_at (curr) } } # [doc = " This will pop the element from the list."] # [doc = ""] # [doc = " Complexity is worst-case *O*(1)."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use heapless::sorted_linked_list::{Max, SortedLinkedList};"] # [doc = " let mut ll: SortedLinkedList<_, Max, 3, u8> = SortedLinkedList::new_u8();"] # [doc = ""] # [doc = " ll.push(1).unwrap();"] # [doc = " ll.push(2).unwrap();"] # [doc = " ll.push(3).unwrap();"] # [doc = ""] # [doc = " // Find a value and update it"] # [doc = " let mut find = ll.find_mut(|v| *v == 2).unwrap();"] # [doc = " find.pop();"] # [doc = ""] # [doc = " assert_eq!(ll.pop(), Some(3));"] # [doc = " assert_eq!(ll.pop(), Some(1));"] # [doc = " assert_eq!(ll.pop(), None);"] # [doc = " ```"] # [inline] pub fn pop (mut self) -> T { self . pop_internal () } # [doc = " This will resort the element into the correct position in the list if needed. The resorting"] # [doc = " will only happen if the element has been accessed mutably."] # [doc = ""] # [doc = " Same as calling `drop`."] # [doc = ""] # [doc = " Complexity is worst-case *O*(n)."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use heapless::sorted_linked_list::{Max, SortedLinkedList};"] # [doc = " let mut ll: SortedLinkedList<_, Max, 3, u8> = SortedLinkedList::new_u8();"] # [doc = ""] # [doc = " ll.push(1).unwrap();"] # [doc = " ll.push(2).unwrap();"] # [doc = " ll.push(3).unwrap();"] # [doc = ""] # [doc = " let mut find = ll.find_mut(|v| *v == 2).unwrap();"] # [doc = " find.finish(); // No resort, we did not access the value."] # [doc = ""] # [doc = " let mut find = ll.find_mut(|v| *v == 2).unwrap();"] # [doc = " *find += 1000;"] # [doc = " find.finish(); // Will resort, we accessed (and updated) the value."] # [doc = ""] # [doc = " assert_eq!(ll.pop(), Some(1002));"] # [doc = " assert_eq!(ll.pop(), Some(3));"] # [doc = " assert_eq!(ll.pop(), Some(1));"] # [doc = " assert_eq!(ll.pop(), None);"] # [doc = " ```"] # [inline] pub fn finish (self) { drop (self) ; } }
    };
}

impl_442!();