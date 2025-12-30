// Generated macro for JniError (enum)
macro_rules! Depcrate_errorsJniError {
() => {
// Module: crate::errors
// Provides: {"JniError"}
// Dependencies: {}
# [derive (Debug , Error)] pub enum JniError { # [error ("Unknown error")] Unknown , # [error ("Current thread is not attached to the Java VM")] ThreadDetached , # [error ("JNI version error")] WrongVersion , # [error ("Not enough memory")] NoMemory , # [error ("VM already created")] AlreadyCreated , # [error ("Invalid arguments")] InvalidArguments , # [error ("Error code {0}")] Other (sys :: jint) , }
};
}
