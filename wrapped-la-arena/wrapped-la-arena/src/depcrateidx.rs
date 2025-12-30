// Generated macro for Idx (struct)
macro_rules! DepcrateIdx {
() => {
// Module: crate
// Provides: {"Idx"}
// Dependencies: {}
# [doc = " The index of a value allocated in an arena that holds `T`s."] pub struct Idx < T > { raw : RawIdx , _ty : PhantomData < fn () -> T > , }
};
}
