// Generated macro for impl_565 (impl)
macro_rules! Depcrate_spscimpl_565 {
() => {
// Module: crate::spsc
// Provides: {"impl_565"}
// Dependencies: {}
impl < T > Producer < '_ , T > { # [doc = " Adds an `item` to the end of the queue, returns back the `item` if the queue is full."] # [inline] pub fn enqueue (& mut self , item : T) -> Result < () , T > { unsafe { self . rb . inner_enqueue (item) } } # [doc = " Adds an `item` to the end of the queue, without checking if the queue is full."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " See [`Queue::enqueue_unchecked`]."] # [inline] pub unsafe fn enqueue_unchecked (& mut self , item : T) { self . rb . inner_enqueue_unchecked (item) ; } # [doc = " Returns if there is any space to enqueue a new item. When this returns true, at"] # [doc = " least the first subsequent enqueue will succeed."] # [inline] pub fn ready (& self) -> bool { ! self . rb . is_full () } # [doc = " Returns the number of elements in the queue."] # [inline] pub fn len (& self) -> usize { self . rb . len () } # [doc = " Returns whether the queue is empty."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use heapless::spsc::Queue;"] # [doc = ""] # [doc = " let mut queue: Queue<u8, 235> = Queue::new();"] # [doc = " let (mut producer, mut consumer) = queue.split();"] # [doc = " assert!(producer.is_empty());"] # [doc = " ```"] # [inline] pub fn is_empty (& self) -> bool { self . len () == 0 } # [doc = " Returns the maximum number of elements the queue can hold."] # [inline] pub fn capacity (& self) -> usize { self . rb . capacity () } }
};
}
