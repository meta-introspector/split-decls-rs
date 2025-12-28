macro_rules! deps {
    () => {
        BufMut!();
    };
}

macro_rules! Limit {
    () => {
        deps!();
        # [doc = " A `BufMut` adapter which limits the amount of bytes that can be written"] # [doc = " to an underlying buffer."] # [derive (Debug)] pub struct Limit < T > { inner : T , limit : usize , }
    };
}

Limit!();