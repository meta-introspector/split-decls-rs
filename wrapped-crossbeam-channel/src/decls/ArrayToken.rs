macro_rules! deps {
    () => {
        Slot!();
    };
}

macro_rules! ArrayToken {
    () => {
        deps!();
        # [doc = " The token type for the array flavor."] # [derive (Debug)] pub (crate) struct ArrayToken { # [doc = " Slot to read from or write to."] slot : * const u8 , # [doc = " Stamp to store into the slot after reading or writing."] stamp : usize , }
    };
}

ArrayToken!();