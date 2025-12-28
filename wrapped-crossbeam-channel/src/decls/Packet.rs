macro_rules! Packet {
    () => {
        # [doc = " A slot for passing one message from a sender to a receiver."] struct Packet < T > { # [doc = " Equals `true` if the packet is allocated on the stack."] on_stack : bool , # [doc = " Equals `true` once the packet is ready for reading or writing."] ready : AtomicBool , # [doc = " The message."] msg : UnsafeCell < Option < T > > , }
    };
}

Packet!();