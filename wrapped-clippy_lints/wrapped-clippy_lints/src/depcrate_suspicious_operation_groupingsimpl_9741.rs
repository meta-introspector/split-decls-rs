// Generated macro for impl_9741 (impl)
macro_rules! Depcrate_suspicious_operation_groupingsimpl_9741 {
() => {
// Module: crate::suspicious_operation_groupings
// Provides: {"impl_9741"}
// Dependencies: {}
impl Add for IdentDifference { type Output = IdentDifference ; fn add (self , other : Self) -> Self :: Output { match (self , other) { (Self :: NoDifference , output) | (output , Self :: NoDifference) => output , (Self :: Multiple , _) | (_ , Self :: Multiple) | (Self :: Double (_ , _) , Self :: Single (_)) | (Self :: Single (_) | Self :: Double (_ , _) , Self :: Double (_ , _)) => Self :: Multiple , (Self :: NonIdent , _) | (_ , Self :: NonIdent) => Self :: NonIdent , (Self :: Single (il1) , Self :: Single (il2)) => Self :: Double (il1 , il2) , } } }
};
}
