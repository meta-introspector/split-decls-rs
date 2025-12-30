// Generated macro for IfrEqIdValList (struct)
macro_rules! Depcrate_hiiIfrEqIdValList {
() => {
// Module: crate::hii
// Provides: {"IfrEqIdValList"}
// Dependencies: {}
# [repr (C)] # [derive (Clone , Copy , Debug)] pub struct IfrEqIdValList < const N : usize = 0 > { pub header : IfrOpHeader , pub question_id : QuestionId , pub list_length : u16 , pub value_list : [u16 ; N] , }
};
}
