// Generated macro for CFDictionaryKeyCallBacks (struct)
macro_rules! Depcrate_dictionaryCFDictionaryKeyCallBacks {
() => {
// Module: crate::dictionary
// Provides: {"CFDictionaryKeyCallBacks"}
// Dependencies: {}
# [repr (C)] # [derive (Clone , Copy)] pub struct CFDictionaryKeyCallBacks { pub version : CFIndex , pub retain : CFDictionaryRetainCallBack , pub release : CFDictionaryReleaseCallBack , pub copyDescription : CFDictionaryCopyDescriptionCallBack , pub equal : CFDictionaryEqualCallBack , pub hash : CFDictionaryHashCallBack , }
};
}
