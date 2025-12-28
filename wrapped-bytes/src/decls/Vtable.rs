macro_rules! deps {
    () => {
        Bytes!();
        BytesMut!();
    };
}

macro_rules! Vtable {
    () => {
        deps!();
        pub (crate) struct Vtable { # [doc = " fn(data, ptr, len)"] pub clone : unsafe fn (& AtomicPtr < () > , * const u8 , usize) -> Bytes , # [doc = " fn(data, ptr, len)"] # [doc = ""] # [doc = " `into_*` consumes the `Bytes`, returning the respective value."] pub into_vec : unsafe fn (& AtomicPtr < () > , * const u8 , usize) -> Vec < u8 > , pub into_mut : unsafe fn (& AtomicPtr < () > , * const u8 , usize) -> BytesMut , # [doc = " fn(data)"] pub is_unique : unsafe fn (& AtomicPtr < () >) -> bool , # [doc = " fn(data, ptr, len)"] pub drop : unsafe fn (& mut AtomicPtr < () > , * const u8 , usize) , }
    };
}

Vtable!()