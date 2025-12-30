// Generated macro for impl_546 (impl)
macro_rules! Depcrate_spscimpl_546 {
() => {
// Module: crate::spsc
// Provides: {"impl_546"}
// Dependencies: {}
impl < T , S , S2 > PartialEq < QueueInner < T , S2 > > for QueueInner < T , S > where T : PartialEq , S : Storage , S2 : Storage , { fn eq (& self , other : & QueueInner < T , S2 >) -> bool { self . len () == other . len () && self . iter () . zip (other . iter ()) . all (| (v1 , v2) | v1 == v2) } }
};
}
