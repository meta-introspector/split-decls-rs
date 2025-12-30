// Generated macro for CFArrayCallBacks (struct)
macro_rules! Depcrate_arrayCFArrayCallBacks {
() => {
// Module: crate::array
// Provides: {"CFArrayCallBacks"}
// Dependencies: {}
# [repr (C)] # [derive (Clone , Copy , Debug)] pub struct CFArrayCallBacks { pub version : CFIndex , pub retain : CFArrayRetainCallBack , pub release : CFArrayReleaseCallBack , pub copyDescription : CFArrayCopyDescriptionCallBack , pub equal : CFArrayEqualCallBack , }
};
}
