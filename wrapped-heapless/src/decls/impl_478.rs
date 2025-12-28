macro_rules! deps {
    () => {
        Consumer!();
    };
}

macro_rules! impl_478 {
    () => {
        deps!();
        impl < T > Consumer < '_ , T > { # [doc = " Returns the item in the front of the queue, or `None` if the queue is empty."] # [inline] pub fn dequeue (& mut self) -> Option < T > { unsafe { self . rb . inner_dequeue () } } # [doc = " Returns the item in the front of the queue, without checking if there are elements in the"] # [doc = " queue."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " See [`Queue::dequeue_unchecked`]."] # [inline] pub unsafe fn dequeue_unchecked (& mut self) -> T { self . rb . inner_dequeue_unchecked () } # [doc = " Returns if there are any items to dequeue. When this returns `true`, at least the"] # [doc = " first subsequent dequeue will succeed."] # [inline] pub fn ready (& self) -> bool { ! self . rb . is_empty () } # [doc = " Returns the number of elements in the queue."] # [inline] pub fn len (& self) -> usize { self . rb . len () } # [doc = " Returns whether the queue is empty."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use heapless::spsc::Queue;"] # [doc = ""] # [doc = " let mut queue: Queue<u8, 235> = Queue::new();"] # [doc = " let (mut producer, mut consumer) = queue.split();"] # [doc = " assert!(consumer.is_empty());"] # [doc = " ```"] # [inline] pub fn is_empty (& self) -> bool { self . len () == 0 } # [doc = " Returns the maximum number of elements the queue can hold."] # [inline] pub fn capacity (& self) -> usize { self . rb . capacity () } # [doc = " Returns the item in the front of the queue without dequeuing, or `None` if the queue is"] # [doc = " empty."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use heapless::spsc::Queue;"] # [doc = ""] # [doc = " let mut queue: Queue<u8, 235> = Queue::new();"] # [doc = " let (mut producer, mut consumer) = queue.split();"] # [doc = " assert_eq!(None, consumer.peek());"] # [doc = " producer.enqueue(1);"] # [doc = " assert_eq!(Some(&1), consumer.peek());"] # [doc = " assert_eq!(Some(1), consumer.dequeue());"] # [doc = " assert_eq!(None, consumer.peek());"] # [doc = " ```"] # [inline] pub fn peek (& self) -> Option < & T > { self . rb . peek () } }
    };
}

impl_478!();