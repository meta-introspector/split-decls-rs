macro_rules! deps {
    () => {
        Producer!();
        Queue!();
        Consumer!();
        QueueView!();
    };
}

macro_rules! impl_457 {
    () => {
        deps!();
        impl < T > QueueView < T > { # [doc = " Splits a queue into producer and consumer endpoints."] # [doc = ""] # [doc = " Unlike [`Queue::split`](), this method can be used in a `const` context"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " Create and split a queue at compile time, and pass it to the main"] # [doc = " function and an interrupt handler via a mutex at runtime."] # [doc = ""] # [doc = " ```"] # [doc = " use core::cell::RefCell;"] # [doc = ""] # [doc = " use critical_section::Mutex;"] # [doc = " use heapless::spsc::{Consumer, Producer, Queue, QueueView};"] # [doc = ""] # [doc = " static PC: ("] # [doc = "     Mutex<RefCell<Option<Producer<'_, ()>>>>,"] # [doc = "     Mutex<RefCell<Option<Consumer<'_, ()>>>>,"] # [doc = " ) = {"] # [doc = "     static mut Q: &mut QueueView<()> = &mut Queue::<(), 4>::new();"] # [doc = "     // SAFETY: `Q` is only accessible in this scope."] # [doc = "     #[allow(static_mut_refs)]"] # [doc = "     let (p, c) = unsafe { Q.split_const() };"] # [doc = ""] # [doc = "     ("] # [doc = "         Mutex::new(RefCell::new(Some(p))),"] # [doc = "         Mutex::new(RefCell::new(Some(c))),"] # [doc = "     )"] # [doc = " };"] # [doc = ""] # [doc = " fn interrupt() {"] # [doc = "     let mut producer = {"] # [doc = "         static mut P: Option<Producer<'_, ()>> = None;"] # [doc = "         // SAFETY: Mutable access to `P` is allowed exclusively in this scope"] # [doc = "         // and `interrupt` cannot be called directly or preempt itself."] # [doc = "         unsafe { &mut P }"] # [doc = "     }"] # [doc = "     .get_or_insert_with(|| {"] # [doc = "         critical_section::with(|cs| PC.0.borrow_ref_mut(cs).take().unwrap())"] # [doc = "     });"] # [doc = ""] # [doc = "     producer.enqueue(()).unwrap();"] # [doc = " }"] # [doc = ""] # [doc = " fn main() {"] # [doc = "     let mut consumer = critical_section::with(|cs| PC.1.borrow_ref_mut(cs).take().unwrap());"] # [doc = ""] # [doc = "     // Interrupt occurs."] # [doc = " #   interrupt();"] # [doc = ""] # [doc = "     consumer.dequeue().unwrap();"] # [doc = " }"] # [doc = " ```"] pub const fn split_const (& mut self) -> (Producer < '_ , T > , Consumer < '_ , T >) { (Producer { rb : self } , Consumer { rb : self }) } }
    };
}

impl_457!()