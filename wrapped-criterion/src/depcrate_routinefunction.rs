// Generated macro for Function (struct)
macro_rules! Depcrate_routineFunction {
() => {
// Module: crate::routine
// Provides: {"Function"}
// Dependencies: {}
pub struct Function < M : Measurement , F , T > where F : FnMut (& mut Bencher < '_ , M > , & T) , T : ? Sized , { f : F , _phantom : PhantomData < T > , _phamtom2 : PhantomData < M > , }
};
}
