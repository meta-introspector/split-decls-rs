// Generated macro for Method (struct)
macro_rules! Depcrate_leavesMethod {
() => {
// Module: crate::leaves
// Provides: {"Method"}
// Dependencies: {}
# [derive (Debug)] # [doc = " A D-Bus Method."] pub struct Method < M : MethodType < D > , D : DataType > { cb : DebugMethod < M , D > , data : D :: Method , name : Member < 'static > , i_args : Vec < Argument > , o_args : Vec < Argument > , anns : Annotations , }
};
}
