// Generated macro for WriteResult (enum)
macro_rules! Depcrate_seWriteResult {
() => {
// Module: crate::se
// Provides: {"WriteResult"}
// Dependencies: {}
# [doc = " Classification of the type written by the serializer."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum WriteResult { # [doc = " Text with insignificant spaces was written, for example a number. Adding indent to the"] # [doc = " serialized data does not change meaning of the data."] Text , # [doc = " The XML tag was written. Adding indent to the serialized data does not change meaning of the data."] Element , # [doc = " Nothing was written (i. e. serialized type not represented in XML a all). Adding indent to the"] # [doc = " serialized data does not change meaning of the data. This is returned for units, unit structs"] # [doc = " and unit variants."] Nothing , # [doc = " Text with significant spaces was written, for example a string. Adding indent to the"] # [doc = " serialized data may change meaning of the data."] SensitiveText , # [doc = " `None` was serialized and nothing was written. `None` does not represented in XML,"] # [doc = " but adding indent after it may change meaning of the data."] SensitiveNothing , }
};
}
