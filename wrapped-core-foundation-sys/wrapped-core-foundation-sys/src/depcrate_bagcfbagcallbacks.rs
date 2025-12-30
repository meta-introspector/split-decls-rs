// Generated macro for CFBagCallBacks (struct)
macro_rules! Depcrate_bagCFBagCallBacks {
() => {
// Module: crate::bag
// Provides: {"CFBagCallBacks"}
// Dependencies: {}
# [repr (C)] # [derive (Debug , Clone , Copy)] pub struct CFBagCallBacks { pub version : CFIndex , pub retain : CFBagRetainCallBack , pub release : CFBagReleaseCallBack , pub copyDescription : CFBagCopyDescriptionCallBack , pub equal : CFBagEqualCallBack , pub hash : CFBagHashCallBack , }
};
}
