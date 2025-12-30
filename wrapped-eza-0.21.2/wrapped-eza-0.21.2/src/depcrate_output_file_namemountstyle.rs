// Generated macro for MountStyle (enum)
macro_rules! Depcrate_output_file_nameMountStyle {
() => {
// Module: crate::output::file_name
// Provides: {"MountStyle"}
// Dependencies: {}
# [doc = " When displaying a directory name, there needs to be some way to handle"] # [doc = " mount details, depending on how long the resulting Cell can be."] # [derive (PartialEq , Debug , Copy , Clone)] enum MountStyle { # [doc = " Just display the directory names."] JustDirectoryNames , # [doc = " Display mount points as directories and include information about"] # [doc = " the filesystem that's mounted there."] MountInfo , }
};
}
