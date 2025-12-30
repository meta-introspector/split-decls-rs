// Generated macro for macro_33 (macro)
macro_rules! Depcrate_generatedmacro_33 {
() => {
// Module: crate::generated
// Provides: {"macro_33"}
// Dependencies: {}
extern_protocol ! (# [doc = " [Apple's documentation](https://developer.apple.com/documentation/automaticassessmentconfiguration/aeassessmentsessiondelegate?language=objc)"] pub unsafe trait AEAssessmentSessionDelegate : NSObjectProtocol { # [optional] # [unsafe (method (assessmentSessionDidBegin :))] # [unsafe (method_family = none)] unsafe fn assessmentSessionDidBegin (& self , session : & AEAssessmentSession) ; # [optional] # [unsafe (method (assessmentSession : failedToBeginWithError :))] # [unsafe (method_family = none)] unsafe fn assessmentSession_failedToBeginWithError (& self , session : & AEAssessmentSession , error : & NSError ,) ; # [optional] # [unsafe (method (assessmentSession : wasInterruptedWithError :))] # [unsafe (method_family = none)] unsafe fn assessmentSession_wasInterruptedWithError (& self , session : & AEAssessmentSession , error : & NSError ,) ; # [optional] # [unsafe (method (assessmentSessionDidEnd :))] # [unsafe (method_family = none)] unsafe fn assessmentSessionDidEnd (& self , session : & AEAssessmentSession) ; # [optional] # [unsafe (method (assessmentSessionDidUpdate :))] # [unsafe (method_family = none)] unsafe fn assessmentSessionDidUpdate (& self , session : & AEAssessmentSession) ; # [optional] # [unsafe (method (assessmentSession : failedToUpdateToConfiguration : error :))] # [unsafe (method_family = none)] unsafe fn assessmentSession_failedToUpdateToConfiguration_error (& self , session : & AEAssessmentSession , configuration : & AEAssessmentConfiguration , error : & NSError ,) ; }) ;
};
}
