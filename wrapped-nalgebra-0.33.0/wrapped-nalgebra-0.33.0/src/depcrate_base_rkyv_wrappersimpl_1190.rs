// Generated macro for impl_1190 (impl)
macro_rules! Depcrate_base_rkyv_wrappersimpl_1190 {
() => {
// Module: crate::base::rkyv_wrappers
// Provides: {"impl_1190"}
// Dependencies: {}
impl < OT : ? Sized , NT : ? Sized , D : Fallible + ? Sized > DeserializeWith < PhantomData < NT > , PhantomData < OT > , D > for CustomPhantom < NT > { # [inline] fn deserialize_with (_ : & PhantomData < NT > , _ : & mut D) -> Result < PhantomData < OT > , D :: Error > { Ok (PhantomData) } }
};
}
