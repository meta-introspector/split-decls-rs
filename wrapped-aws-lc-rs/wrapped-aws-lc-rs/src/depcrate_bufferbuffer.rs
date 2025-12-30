// Generated macro for Buffer (struct)
macro_rules! Depcrate_bufferBuffer {
() => {
// Module: crate::buffer
// Provides: {"Buffer"}
// Dependencies: {}
# [doc = " This is a buffer type for some data exposed by various APIs in this crate."] # [doc = ""] # [doc = " `T` acts as a discriminant between different kinds of data."] # [doc = ""] # [doc = " The buffer will be zeroed on drop if it is owned."] pub struct Buffer < 'a , T > (Cow < 'a , [u8] > , PhantomData < T >) ;
};
}
