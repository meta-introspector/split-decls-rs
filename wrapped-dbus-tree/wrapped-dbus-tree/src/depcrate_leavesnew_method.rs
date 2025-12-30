// Generated macro for new_method (function)
macro_rules! Depcrate_leavesnew_method {
() => {
// Module: crate::leaves
// Provides: {"new_method"}
// Dependencies: {}
pub fn new_method < M : MethodType < D > , D : DataType > (n : Member < 'static > , data : D :: Method , cb : Box < M :: Method >) -> Method < M , D > { Method { name : n , i_args : vec ! () , o_args : vec ! () , anns : Annotations :: new () , cb : DebugMethod (cb) , data : data } }
};
}
