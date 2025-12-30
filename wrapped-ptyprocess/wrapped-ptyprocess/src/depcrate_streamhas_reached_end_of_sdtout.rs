// Generated macro for has_reached_end_of_sdtout (function)
macro_rules! Depcrate_streamhas_reached_end_of_sdtout {
() => {
// Module: crate::stream
// Provides: {"has_reached_end_of_sdtout"}
// Dependencies: {}
# [doc = " PTY may doesn't have anything to read but the process is not DEAD,"] # [doc = " and this erorr may be returned."] fn has_reached_end_of_sdtout (err : & std :: io :: Error) -> bool { err . raw_os_error () == Some (5) }
};
}
