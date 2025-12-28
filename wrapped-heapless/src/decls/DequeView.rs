macro_rules! deps {
    () => {
        DequeInner!();
        Deque!();
    };
}

macro_rules! DequeView {
    () => {
        deps!();
        # [doc = " A double-ended queue with dynamic capacity."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use heapless::deque::{Deque, DequeView};"] # [doc = ""] # [doc = " // A deque with a fixed capacity of 8 elements allocated on the stack"] # [doc = " let mut deque_buf = Deque::<_, 8>::new();"] # [doc = ""] # [doc = " // A DequeView can be obtained through unsized coercion of a `Deque`"] # [doc = " let deque: &mut DequeView<_> = &mut deque_buf;"] # [doc = ""] # [doc = " // You can use it as a good old FIFO queue."] # [doc = " deque.push_back(1);"] # [doc = " deque.push_back(2);"] # [doc = " assert_eq!(deque.storage_len(), 2);"] # [doc = ""] # [doc = " assert_eq!(deque.pop_front(), Some(1));"] # [doc = " assert_eq!(deque.pop_front(), Some(2));"] # [doc = " assert_eq!(deque.storage_len(), 0);"] # [doc = ""] # [doc = " // DequeView is double-ended, you can push and pop from the front and back."] # [doc = " deque.push_back(1);"] # [doc = " deque.push_front(2);"] # [doc = " deque.push_back(3);"] # [doc = " deque.push_front(4);"] # [doc = " assert_eq!(deque.pop_front(), Some(4));"] # [doc = " assert_eq!(deque.pop_front(), Some(2));"] # [doc = " assert_eq!(deque.pop_front(), Some(1));"] # [doc = " assert_eq!(deque.pop_front(), Some(3));"] # [doc = ""] # [doc = " // You can iterate it, yielding all the elements front-to-back."] # [doc = " for x in deque {"] # [doc = "     println!(\"{}\", x);"] # [doc = " }"] # [doc = " ```"] pub type DequeView < T > = DequeInner < T , ViewVecStorage < T > > ;
    };
}

DequeView!()