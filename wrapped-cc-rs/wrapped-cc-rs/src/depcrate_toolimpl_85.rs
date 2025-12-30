// Generated macro for impl_85 (impl)
macro_rules! Depcrate_toolimpl_85 {
() => {
// Module: crate::tool
// Provides: {"impl_85"}
// Dependencies: {}
impl ToolFamily { # [doc = " What the flag to request debug info for this family of tools look like"] pub (crate) fn add_debug_flags (& self , cmd : & mut Tool , dwarf_version : Option < u32 >) { match * self { ToolFamily :: Msvc { .. } => { cmd . push_cc_arg ("-Z7" . into ()) ; } ToolFamily :: Gnu | ToolFamily :: Clang { .. } => { cmd . push_cc_arg (dwarf_version . map_or_else (| | "-g" . into () , | v | format ! ("-gdwarf-{v}")) . into () ,) ; } } } # [doc = " What the flag to force frame pointers."] pub (crate) fn add_force_frame_pointer (& self , cmd : & mut Tool) { match * self { ToolFamily :: Gnu | ToolFamily :: Clang { .. } => { cmd . push_cc_arg ("-fno-omit-frame-pointer" . into ()) ; } _ => () , } } # [doc = " What the flags to enable all warnings"] pub (crate) fn warnings_flags (& self) -> & 'static str { match * self { ToolFamily :: Msvc { .. } => "-W4" , ToolFamily :: Gnu | ToolFamily :: Clang { .. } => "-Wall" , } } # [doc = " What the flags to enable extra warnings"] pub (crate) fn extra_warnings_flags (& self) -> Option < & 'static str > { match * self { ToolFamily :: Msvc { .. } => None , ToolFamily :: Gnu | ToolFamily :: Clang { .. } => Some ("-Wextra") , } } # [doc = " What the flag to turn warning into errors"] pub (crate) fn warnings_to_errors_flag (& self) -> & 'static str { match * self { ToolFamily :: Msvc { .. } => "-WX" , ToolFamily :: Gnu | ToolFamily :: Clang { .. } => "-Werror" , } } pub (crate) fn verbose_stderr (& self) -> bool { matches ! (* self , ToolFamily :: Clang { .. }) } }
};
}
