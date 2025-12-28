macro_rules! Slot {
    () => {
        # [doc = " A slot in a block."] struct Slot < T > { # [doc = " The message."] msg : UnsafeCell < MaybeUninit < T > > , # [doc = " The state of the slot."] state : AtomicUsize , }
    };
}

Slot!()