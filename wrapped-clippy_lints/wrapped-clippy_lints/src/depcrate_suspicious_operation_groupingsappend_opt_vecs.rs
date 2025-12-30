// Generated macro for append_opt_vecs (function)
macro_rules! Depcrate_suspicious_operation_groupingsappend_opt_vecs {
() => {
// Module: crate::suspicious_operation_groupings
// Provides: {"append_opt_vecs"}
// Dependencies: {}
fn append_opt_vecs < A > (target_opt : Option < Vec < A > > , source_opt : Option < Vec < A > >) -> Option < Vec < A > > { match (target_opt , source_opt) { (Some (mut target) , Some (source)) => { target . reserve (source . len ()) ; for op in source { target . push (op) ; } Some (target) } , (Some (v) , None) | (None , Some (v)) => Some (v) , (None , None) => None , } }
};
}
