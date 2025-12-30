// Generated macro for DoublePipeline (struct)
macro_rules! DepcrateDoublePipeline {
() => {
// Module: crate
// Provides: {"DoublePipeline"}
// Dependencies: {}
# [doc = " KBKDF in Double-Pipeline Mode."] pub struct DoublePipeline < Prf , K , R = U32 > where Prf : Mac , { _marker : PhantomData < (Prf , K , R) > , }
};
}
