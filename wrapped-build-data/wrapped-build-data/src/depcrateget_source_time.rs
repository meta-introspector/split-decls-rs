// Generated macro for get_source_time (function)
macro_rules! Depcrateget_source_time {
() => {
// Module: crate
// Provides: {"get_source_time"}
// Dependencies: {}
# [doc = " Gets the modification time of the source code."] # [doc = ""] # [doc = " Reads the"] # [doc = " [`SOURCE_DATE_EPOCH`](https://reproducible-builds.org/docs/source-date-epoch/)"] # [doc = " env var if set.  Otherwise, runs `git` to get the value."] # [doc = ""] # [doc = " # Errors"] # [doc = " Returns an error when:"] # [doc = " - `SOURCE_DATE_EPOCH` is non-empty and cannot be parsed as an `u64`"] # [doc = " - it failed to execute `git`"] # [doc = " - `git` exited with non-zero status"] # [doc = " - `git` wrote stdout data in an unexpected format"] pub fn get_source_time () -> Result < u64 , String > { let value = SOURCE_EPOCH_SECONDS . load (Ordering :: Acquire) ; if value != u64 :: MAX { return Ok (value) ; } let value : u64 = if let Some (string) = get_env ("SOURCE_DATE_EPOCH") ? { string . parse () . map_err (| _ | format ! ("error parsing env var as u64: SOURCE_DATE_EPOCH='{value}'")) ? } else { let stdout = exec ("git" , & ["log" , "--no-show-signature" , "-1" , "--pretty=%ct"]) ? ; stdout . parse () . map_err (| _ | { format ! ("failed parsing output of 'git log -1 --pretty=%ct' as u64: '{}'" , escape_ascii (& stdout)) }) ? } ; SOURCE_EPOCH_SECONDS . store (value , Ordering :: Release) ; Ok (value) }
};
}
