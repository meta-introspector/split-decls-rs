// Generated macro for AudioFileFDFTableExtended (struct)
macro_rules! Depcrate_generatedAudioFileFDFTableExtended {
() => {
// Module: crate::generated
// Provides: {"AudioFileFDFTableExtended"}
// Dependencies: {}
# [doc = " [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/audiofilefdftableextended?language=objc)"] # [cfg (all (feature = "AudioFile" , feature = "objc2-core-audio-types"))] # [repr (C)] # [allow (unpredictable_function_pointer_comparisons)] # [derive (Clone , Copy , Debug , PartialEq)] pub struct AudioFileFDFTableExtended { pub mComponentStorage : NonNull < c_void > , pub mReadBytesFDF : ReadBytesFDF , pub mWriteBytesFDF : WriteBytesFDF , pub mReadPacketsFDF : ReadPacketsFDF , pub mWritePacketsFDF : WritePacketsFDF , pub mGetPropertyInfoFDF : GetPropertyInfoFDF , pub mGetPropertyFDF : GetPropertyFDF , pub mSetPropertyFDF : SetPropertyFDF , pub mCountUserDataFDF : CountUserDataFDF , pub mGetUserDataSizeFDF : GetUserDataSizeFDF , pub mGetUserDataFDF : GetUserDataFDF , pub mSetUserDataFDF : SetUserDataFDF , pub mReadPacketDataFDF : ReadPacketDataFDF , }
};
}
