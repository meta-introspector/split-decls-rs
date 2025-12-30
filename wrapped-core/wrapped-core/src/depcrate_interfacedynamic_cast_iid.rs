// Generated macro for DYNAMIC_CAST_IID (const)
macro_rules! Depcrate_interfaceDYNAMIC_CAST_IID {
() => {
// Module: crate::interface
// Provides: {"DYNAMIC_CAST_IID"}
// Dependencies: {}
# [doc = " This IID identifies a special protocol, used by [`Interface::cast_to_any`]. This is _not_"] # [doc = " an ordinary COM interface; it uses special lifetime rules and a larger interface pointer."] # [doc = " See the comments on [`Interface::cast_to_any`]."] # [doc (hidden)] pub const DYNAMIC_CAST_IID : GUID = GUID :: from_u128 (0xae49d5cb_143f_431c_874c_2729336e4eca) ;
};
}
