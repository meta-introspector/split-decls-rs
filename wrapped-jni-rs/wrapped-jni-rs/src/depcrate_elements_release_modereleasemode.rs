// Generated macro for ReleaseMode (enum)
macro_rules! Depcrate_elements_release_modeReleaseMode {
() => {
// Module: crate::elements::release_mode
// Provides: {"ReleaseMode"}
// Dependencies: {}
# [doc = " ReleaseMode"] # [doc = ""] # [doc = " This defines the release mode of [`AutoElements`] (and [`AutoElementsCritical`]) resources, and"] # [doc = " related release array functions."] # [derive (Clone , Copy , Debug)] # [repr (i32)] pub enum ReleaseMode { # [doc = " Copy back the content and free the elems buffer. For read-only access, prefer"] # [doc = " [`NoCopyBack`](ReleaseMode::NoCopyBack)."] CopyBack = 0 , # [doc = " Free the buffer without copying back the possible changes."] NoCopyBack = JNI_ABORT , }
};
}
