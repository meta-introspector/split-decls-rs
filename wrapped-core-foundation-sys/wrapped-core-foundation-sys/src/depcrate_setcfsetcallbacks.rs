// Generated macro for CFSetCallBacks (struct)
macro_rules! Depcrate_setCFSetCallBacks {
() => {
// Module: crate::set
// Provides: {"CFSetCallBacks"}
// Dependencies: {}
# [repr (C)] # [derive (Clone , Copy)] pub struct CFSetCallBacks { pub version : CFIndex , pub retain : CFSetRetainCallBack , pub release : CFSetReleaseCallBack , pub copyDescription : CFSetCopyDescriptionCallBack , pub equal : CFSetEqualCallBack , pub hash : CFSetHashCallBack , }
};
}
