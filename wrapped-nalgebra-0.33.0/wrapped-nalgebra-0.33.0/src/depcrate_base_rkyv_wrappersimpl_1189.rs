// Generated macro for impl_1189 (impl)
macro_rules! Depcrate_base_rkyv_wrappersimpl_1189 {
() => {
// Module: crate::base::rkyv_wrappers
// Provides: {"impl_1189"}
// Dependencies: {}
impl < OT : ? Sized , NT : ? Sized , S : Fallible + ? Sized > SerializeWith < PhantomData < OT > , S > for CustomPhantom < NT > { # [inline] fn serialize_with (_ : & PhantomData < OT > , _ : & mut S) -> Result < Self :: Resolver , S :: Error > { Ok (()) } }
};
}
