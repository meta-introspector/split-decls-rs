// Generated macro for CVTimeStamp (struct)
macro_rules! Depcrate_quartzcoreCVTimeStamp {
() => {
// Module: crate::quartzcore
// Provides: {"CVTimeStamp"}
// Dependencies: {}
# [repr (C)] # [derive (Clone , Copy)] pub struct CVTimeStamp { pub version : u32 , pub videoTimeScale : i32 , pub videoTime : i64 , pub hostTime : u64 , pub rateScalar : f64 , pub videoRefreshPeriod : i64 , pub smpteTime : CVSMPTETime , pub flags : u64 , pub reserved : u64 , }
};
}
