// rustc_infer module stub
pub mod infer {
    pub struct TyCtxtInferExt;
    pub struct InferCtxt;
    pub struct NllRegionVariableOrigin;
    pub mod canonical {}
    pub mod at {}
    pub mod region_constraints {}
}

pub mod traits {
    pub mod solve {
        pub struct Goal;
    }
    pub struct ObligationCauseCode;
}
