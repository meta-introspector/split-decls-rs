// Generated macro for Sleep (trait)
macro_rules! Depcrate_rt_timerSleep {
() => {
// Module: crate::rt::timer
// Provides: {"Sleep"}
// Dependencies: {}
# [doc = " A future returned by a `Timer`."] pub trait Sleep : Send + Sync + Future < Output = () > { # [doc (hidden)] # [doc = " This method is private and can not be implemented by downstream crate"] fn __type_id (& self , _ : private :: Sealed) -> TypeId where Self : 'static , { TypeId :: of :: < Self > () } }
};
}
