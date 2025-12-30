// Generated macro for IterMut (struct)
macro_rules! DepcrateIterMut {
() => {
// Module: crate
// Provides: {"IterMut"}
// Dependencies: {}
# [doc = " Mutable arena iterator."] # [doc = ""] # [doc = " This struct is created by the [`iter_mut`](struct.Arena.html#method.iter_mut) method on [Arenas](struct.Arena.html)."] pub struct IterMut < 'a , T : 'a > { chunks : & 'a mut ChunkList < T > , state : IterMutState < 'a , T > , }
};
}
