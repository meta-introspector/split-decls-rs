// Generated macro for new_property (function)
macro_rules! Depcrate_leavesnew_property {
() => {
// Module: crate::leaves
// Provides: {"new_property"}
// Dependencies: {}
pub fn new_property < M : MethodType < D > , D : DataType > (n : String , sig : Signature < 'static > , data : D :: Property) -> Property < M , D > { Property { name : n , emits : EmitsChangedSignal :: True , auto_emit : true , rw : Access :: Read , sig : sig , anns : Annotations :: new () , set_cb : None , get_cb : None , data : data } }
};
}
