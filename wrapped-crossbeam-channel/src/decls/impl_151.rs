macro_rules! deps {
    () => {
        Packet!();
    };
}

macro_rules! impl_151 {
    () => {
        deps!();
        impl < T > Packet < T > { # [doc = " Creates an empty packet on the stack."] fn empty_on_stack () -> Self { Self { on_stack : true , ready : AtomicBool :: new (false) , msg : UnsafeCell :: new (None) , } } # [doc = " Creates an empty packet on the heap."] fn empty_on_heap () -> Box < Self > { Box :: new (Self { on_stack : false , ready : AtomicBool :: new (false) , msg : UnsafeCell :: new (None) , }) } # [doc = " Creates a packet on the stack, containing a message."] fn message_on_stack (msg : T) -> Self { Self { on_stack : true , ready : AtomicBool :: new (false) , msg : UnsafeCell :: new (Some (msg)) , } } # [doc = " Waits until the packet becomes ready for reading or writing."] fn wait_ready (& self) { let backoff = Backoff :: new () ; while ! self . ready . load (Ordering :: Acquire) { backoff . snooze () ; } } }
    };
}

impl_151!();