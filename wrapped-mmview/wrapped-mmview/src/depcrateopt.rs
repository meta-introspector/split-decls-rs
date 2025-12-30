// Generated macro for Opt (struct)
macro_rules! DepcrateOpt {
() => {
// Module: crate
// Provides: {"Opt"}
// Dependencies: {}
# [derive (Parser , Debug)] struct Opt { file_prefix : PathBuf , # [doc = " Filter to events which occured on the specified thread id"] # [arg (short = 't' , long = "thread-id")] thread_id : Option < u32 > , }
};
}
