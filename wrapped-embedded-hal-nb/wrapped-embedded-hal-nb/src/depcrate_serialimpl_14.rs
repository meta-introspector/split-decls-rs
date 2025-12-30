// Generated macro for impl_14 (impl)
macro_rules! Depcrate_serialimpl_14 {
() => {
// Module: crate::serial
// Provides: {"impl_14"}
// Dependencies: {}
impl < T : Write < Word > + ? Sized , Word : Copy > Write < Word > for & mut T { # [inline] fn write (& mut self , word : Word) -> nb :: Result < () , Self :: Error > { T :: write (self , word) } # [inline] fn flush (& mut self) -> nb :: Result < () , Self :: Error > { T :: flush (self) } }
};
}
