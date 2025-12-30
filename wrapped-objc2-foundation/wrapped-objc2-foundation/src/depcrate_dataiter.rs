// Generated macro for Iter (struct)
macro_rules! Depcrate_dataIter {
() => {
// Module: crate::data
// Provides: {"Iter"}
// Dependencies: {}
# [doc = " An iterator over the bytes in an `NSData`."] # [doc = ""] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " The iteration may panic if the data is mutated while being iterated upon."] # [derive (Debug , Clone)] pub struct Iter < 'a > { p : PhantomData < & 'a NSData > , # [cfg (debug_assertions)] data : & 'a NSData , # [cfg (debug_assertions)] length : usize , bytes : alloc :: vec :: IntoIter < u8 > , }
};
}
