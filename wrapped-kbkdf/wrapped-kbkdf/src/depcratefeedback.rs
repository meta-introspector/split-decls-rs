// Generated macro for Feedback (struct)
macro_rules! DepcrateFeedback {
() => {
// Module: crate
// Provides: {"Feedback"}
// Dependencies: {}
# [doc = " KBKDF in Feedback Mode."] pub struct Feedback < 'a , Prf , K , R = U32 > where Prf : Mac , { iv : Option < & 'a Array < u8 , Prf :: OutputSize > > , _marker : PhantomData < (Prf , K , R) > , }
};
}
