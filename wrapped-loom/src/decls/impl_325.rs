macro_rules! deps {
    () => {
        Thread!();
        ThreadId!();
    };
}

macro_rules! impl_325 {
    () => {
        deps!();
        impl Thread { # [doc = " Returns a unique identifier for this thread"] pub fn id (& self) -> ThreadId { self . id } # [doc = " Returns the (optional) name of this thread"] pub fn name (& self) -> Option < & str > { self . name . as_deref () } # [doc = " Mock implementation of [`std::thread::Thread::unpark`]."] # [doc = ""] # [doc = " Atomically makes the handle's token available if it is not already."] # [doc = ""] # [doc = " Every thread is equipped with some basic low-level blocking support, via"] # [doc = " the [`park`] function and the `unpark()` method. These can be"] # [doc = " used as a more CPU-efficient implementation of a spinlock."] # [doc = ""] # [doc = " See the [park documentation][park] for more details."] pub fn unpark (& self) { rt :: execution (| execution | execution . threads . unpark (self . id . id)) ; } }
    };
}

impl_325!()