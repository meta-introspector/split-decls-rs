macro_rules! deps {
    () => {
        QueueInner!();
        Queue!();
        Storage!();
        UintSize!();
        QueueView!();
    };
}

macro_rules! impl_404 {
    () => {
        deps!();
        impl < T , S : Storage > QueueInner < T , S > { # [doc = " Returns the maximum number of elements the queue can hold."] # [inline] pub fn capacity (& self) -> usize { S :: len (self . buffer . get ()) } # [doc = " Get a reference to the `Queue`, erasing the `N` const-generic."] # [doc = ""] # [doc = ""] # [doc = " ```rust"] # [doc = " # use heapless::mpmc::{Queue, QueueView};"] # [doc = " let queue: Queue<u8, 2> = Queue::new();"] # [doc = " let view: &QueueView<u8> = queue.as_view();"] # [doc = " ```"] # [doc = ""] # [doc = " It is often preferable to do the same through type coerction, since `Queue<T, N>` implements"] # [doc = " `Unsize<QueueView<T>>`:"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use heapless::mpmc::{Queue, QueueView};"] # [doc = " let queue: Queue<u8, 2> = Queue::new();"] # [doc = " let view: &QueueView<u8> = &queue;"] # [doc = " ```"] # [inline] pub fn as_view (& self) -> & QueueView < T > { S :: as_mpmc_view (self) } # [doc = " Get a mutable reference to the `Queue`, erasing the `N` const-generic."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use heapless::mpmc::{Queue, QueueView};"] # [doc = " ##[expect(deprecated)]"] # [doc = " let mut queue: Queue<u8, 2> = Queue::new();"] # [doc = " let view: &mut QueueView<u8> = queue.as_mut_view();"] # [doc = " ```"] # [doc = ""] # [doc = " It is often preferable to do the same through type coerction, since `Queue<T, N>` implements"] # [doc = " `Unsize<QueueView<T>>`:"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use heapless::mpmc::{Queue, QueueView};"] # [doc = " ##[expect(deprecated)]"] # [doc = " let mut queue: Queue<u8, 2> = Queue::new();"] # [doc = " let view: &mut QueueView<u8> = &mut queue;"] # [doc = " ```"] # [inline] pub fn as_mut_view (& mut self) -> & mut QueueView < T > { S :: as_mpmc_mut_view (self) } fn mask (& self) -> UintSize { (S :: len (self . buffer . get ()) - 1) as _ } # [doc = " Returns the item in the front of the queue, or `None` if the queue is empty."] pub fn dequeue (& self) -> Option < T > { unsafe { dequeue (S :: as_ptr (self . buffer . get ()) , & self . dequeue_pos , self . mask ()) } } # [doc = " Adds an `item` to the end of the queue."] # [doc = ""] # [doc = " Returns back the `item` if the queue is full."] pub fn enqueue (& self , item : T) -> Result < () , T > { unsafe { enqueue (S :: as_ptr (self . buffer . get ()) , & self . enqueue_pos , self . mask () , item ,) } } }
    };
}

impl_404!()