// Generated macro for Column (enum)
macro_rules! Depcrate_output_tableColumn {
() => {
// Module: crate::output::table
// Provides: {"Column"}
// Dependencies: {}
# [doc = " A table contains these."] # [derive (Debug , Copy , Clone)] pub enum Column { Permissions , FileSize , Timestamp (TimeType) , # [cfg (unix)] Blocksize , # [cfg (unix)] User , # [cfg (unix)] Group , # [cfg (unix)] HardLinks , # [cfg (unix)] Inode , GitStatus , SubdirGitRepo (bool) , # [cfg (unix)] Octal , # [cfg (unix)] SecurityContext , FileFlags , }
};
}
