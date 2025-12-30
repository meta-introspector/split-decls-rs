// Generated macro for impl_835 (impl)
macro_rules! Depcrate_output_tableimpl_835 {
() => {
// Module: crate::output::table
// Provides: {"impl_835"}
// Dependencies: {}
impl Column { # [doc = " Get the alignment this column should use."] # [cfg (unix)] pub fn alignment (self) -> Alignment { # [allow (clippy :: wildcard_in_or_patterns)] match self { Self :: FileSize | Self :: HardLinks | Self :: Inode | Self :: Blocksize | Self :: GitStatus => { Alignment :: Right } Self :: Timestamp (_) | _ => Alignment :: Left , } } # [cfg (windows)] pub fn alignment (self) -> Alignment { match self { Self :: FileSize | Self :: GitStatus => Alignment :: Right , _ => Alignment :: Left , } } # [doc = " Get the text that should be printed at the top, when the user elects"] # [doc = " to have a header row printed."] pub fn header (self) -> & 'static str { match self { # [cfg (unix)] Self :: Permissions => "Permissions" , # [cfg (windows)] Self :: Permissions => "Mode" , Self :: FileSize => "Size" , Self :: Timestamp (t) => t . header () , # [cfg (unix)] Self :: Blocksize => "Blocksize" , # [cfg (unix)] Self :: User => "User" , # [cfg (unix)] Self :: Group => "Group" , # [cfg (unix)] Self :: HardLinks => "Links" , # [cfg (unix)] Self :: Inode => "inode" , Self :: GitStatus => "Git" , Self :: SubdirGitRepo (_) => "Git Repo" , # [cfg (unix)] Self :: Octal => "Octal" , # [cfg (unix)] Self :: SecurityContext => "Security Context" , Self :: FileFlags => "Flags" , } } }
};
}
