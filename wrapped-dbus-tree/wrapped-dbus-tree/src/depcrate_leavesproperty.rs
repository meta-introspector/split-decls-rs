// Generated macro for Property (struct)
macro_rules! Depcrate_leavesProperty {
() => {
// Module: crate::leaves
// Provides: {"Property"}
// Dependencies: {}
# [derive (Debug)] # [doc = " A D-Bus Property."] pub struct Property < M : MethodType < D > , D : DataType > { name : String , data : D :: Property , sig : Signature < 'static > , emits : EmitsChangedSignal , auto_emit : bool , rw : Access , get_cb : Option < DebugGetProp < M , D > > , set_cb : Option < DebugSetProp < M , D > > , anns : Annotations , }
};
}
