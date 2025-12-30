// Generated macro for CFDictionaryValueCallBacks (struct)
macro_rules! Depcrate_dictionaryCFDictionaryValueCallBacks {
() => {
// Module: crate::dictionary
// Provides: {"CFDictionaryValueCallBacks"}
// Dependencies: {}
# [repr (C)] # [derive (Clone , Copy)] pub struct CFDictionaryValueCallBacks { pub version : CFIndex , pub retain : CFDictionaryRetainCallBack , pub release : CFDictionaryReleaseCallBack , pub copyDescription : CFDictionaryCopyDescriptionCallBack , pub equal : CFDictionaryEqualCallBack , }
};
}
