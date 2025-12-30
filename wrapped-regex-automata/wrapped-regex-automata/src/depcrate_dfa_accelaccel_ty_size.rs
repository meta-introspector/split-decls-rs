// Generated macro for ACCEL_TY_SIZE (const)
macro_rules! Depcrate_dfa_accelACCEL_TY_SIZE {
() => {
// Module: crate::dfa::accel
// Provides: {"ACCEL_TY_SIZE"}
// Dependencies: {}
# [doc = " The size of the unit of representation for accelerators."] # [doc = ""] # [doc = " ACCEL_CAP *must* be a multiple of this size."] const ACCEL_TY_SIZE : usize = core :: mem :: size_of :: < AccelTy > () ;
};
}
