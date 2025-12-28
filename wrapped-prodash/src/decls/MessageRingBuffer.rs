macro_rules! deps {
    () => {
        Message!();
    };
}

macro_rules! MessageRingBuffer {
    () => {
        deps!();
        # [doc = " A ring buffer for messages."] # [derive (Debug , Clone , Eq , PartialEq)] pub struct MessageRingBuffer { pub (crate) buf : Vec < Message > , cursor : usize , total : usize , }
    };
}

MessageRingBuffer!();