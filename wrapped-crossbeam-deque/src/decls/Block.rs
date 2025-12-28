macro_rules! deps {
    () => {
        Slot!();
    };
}

macro_rules! Block {
    () => {
        deps!();
        # [doc = " A block in a linked list."] # [doc = ""] # [doc = " Each block in the list can hold up to `BLOCK_CAP` values."] struct Block < T > { # [doc = " The next block in the linked list."] next : AtomicPtr < Block < T > > , # [doc = " Slots for values."] slots : [Slot < T > ; BLOCK_CAP] , }
    };
}

Block!();