// Generated macro for Autoderef (type)
macro_rules! Depcrate_infer_autoderefAutoderef {
() => {
// Module: crate::infer::autoderef
// Provides: {"Autoderef"}
// Dependencies: {}
pub (crate) type Autoderef < 'a , 'db , Steps = Vec < (Ty < 'db > , AutoderefKind) > > = GeneralAutoderef < 'db , DefaultAutoderefCtx < 'a , 'db > , Steps > ;
};
}
