// Generated macro for impl_1174 (impl)
macro_rules! Depcrate_util_borrowimpl_1174 {
() => {
// Module: crate::util::borrow
// Provides: {"impl_1174"}
// Dependencies: {}
impl < 'a , T : core :: fmt :: Display > core :: fmt :: Display for DumbCow < 'a , T > { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { core :: fmt :: Display :: fmt (core :: ops :: Deref :: deref (self) , f) } }
};
}
