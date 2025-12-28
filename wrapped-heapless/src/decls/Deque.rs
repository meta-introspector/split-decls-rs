macro_rules! deps {
    () => {
        DequeInner!();
    };
}

macro_rules! Deque {
    () => {
        deps!();
        # [doc = " A fixed capacity double-ended queue."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use heapless::Deque;"] # [doc = ""] # [doc = " // A deque with a fixed capacity of 8 elements allocated on the stack"] # [doc = " let mut deque = Deque::<_, 8>::new();"] # [doc = ""] # [doc = " // You can use it as a good old FIFO queue."] # [doc = " deque.push_back(1);"] # [doc = " deque.push_back(2);"] # [doc = " assert_eq!(deque.len(), 2);"] # [doc = ""] # [doc = " assert_eq!(deque.pop_front(), Some(1));"] # [doc = " assert_eq!(deque.pop_front(), Some(2));"] # [doc = " assert_eq!(deque.len(), 0);"] # [doc = ""] # [doc = " // Deque is double-ended, you can push and pop from the front and back."] # [doc = " deque.push_back(1);"] # [doc = " deque.push_front(2);"] # [doc = " deque.push_back(3);"] # [doc = " deque.push_front(4);"] # [doc = " assert_eq!(deque.pop_front(), Some(4));"] # [doc = " assert_eq!(deque.pop_front(), Some(2));"] # [doc = " assert_eq!(deque.pop_front(), Some(1));"] # [doc = " assert_eq!(deque.pop_front(), Some(3));"] # [doc = ""] # [doc = " // You can iterate it, yielding all the elements front-to-back."] # [doc = " for x in &deque {"] # [doc = "     println!(\"{}\", x);"] # [doc = " }"] # [doc = " ```"] pub type Deque < T , const N : usize > = DequeInner < T , OwnedVecStorage < T , N > > ;
    };
}

Deque!();