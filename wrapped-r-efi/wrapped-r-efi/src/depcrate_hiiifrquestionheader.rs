// Generated macro for IfrQuestionHeader (struct)
macro_rules! Depcrate_hiiIfrQuestionHeader {
() => {
// Module: crate::hii
// Provides: {"IfrQuestionHeader"}
// Dependencies: {}
# [repr (C)] # [derive (Clone , Copy)] pub struct IfrQuestionHeader { pub header : IfrStatementHeader , pub question_id : QuestionId , pub var_store_id : VarstoreId , pub var_store_info : IfrQuestionHeaderVarstoreInfo , pub flags : u8 , }
};
}
