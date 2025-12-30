// Generated macro for impl_208 (impl)
macro_rules! Depcrate_ordimpl_208 {
() => {
// Module: crate::ord
// Provides: {"impl_208"}
// Dependencies: {}
impl < T > ValueOrd for T where T : OrdIsValueOrd , { fn value_cmp (& self , other : & Self) -> Result < Ordering > { Ok (self . cmp (other)) } }
};
}
