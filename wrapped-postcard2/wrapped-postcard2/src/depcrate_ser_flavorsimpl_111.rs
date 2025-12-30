// Generated macro for impl_111 (impl)
macro_rules! Depcrate_ser_flavorsimpl_111 {
() => {
// Module: crate::ser::flavors
// Provides: {"impl_111"}
// Dependencies: {}
impl < B > Flavor for Cobs < B > where B : Flavor + IndexMut < usize , Output = u8 > , { type Output = < B as Flavor > :: Output ; # [inline (always)] fn try_push (& mut self , data : u8) -> Result < () > { use PushResult :: * ; match self . cobs . push (data) { AddSingle (n) => self . flav . try_push (n) , ModifyFromStartAndSkip ((idx , mval)) => { self . flav [idx] = mval ; self . flav . try_push (0) } ModifyFromStartAndPushAndSkip ((idx , mval , nval)) => { self . flav [idx] = mval ; self . flav . try_push (nval) ? ; self . flav . try_push (0) } } } fn finalize (mut self) -> Result < Self :: Output > { let (idx , mval) = self . cobs . finalize () ; self . flav [idx] = mval ; self . flav . try_push (0) ? ; self . flav . finalize () } }
};
}
