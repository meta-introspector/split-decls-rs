// Generated macro for IfrRef4 (struct)
macro_rules! Depcrate_hiiIfrRef4 {
() => {
// Module: crate::hii
// Provides: {"IfrRef4"}
// Dependencies: {}
# [repr (C)] # [derive (Clone , Copy)] pub struct IfrRef4 { pub header : IfrOpHeader , pub question : IfrQuestionHeader , pub form_id : FormId , pub question_id : QuestionId , pub form_set_id : crate :: base :: Guid , pub device_path : StringId , }
};
}
