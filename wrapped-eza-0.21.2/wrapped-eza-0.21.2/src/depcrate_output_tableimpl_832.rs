// Generated macro for impl_832 (impl)
macro_rules! Depcrate_output_tableimpl_832 {
() => {
// Module: crate::output::table
// Provides: {"impl_832"}
// Dependencies: {}
impl Columns { pub fn collect (& self , actually_enable_git : bool , git_repos : bool) -> Vec < Column > { let mut columns = Vec :: with_capacity (4) ; if self . inode { # [cfg (unix)] columns . push (Column :: Inode) ; } if self . octal { # [cfg (unix)] columns . push (Column :: Octal) ; } if self . permissions { columns . push (Column :: Permissions) ; } if self . links { # [cfg (unix)] columns . push (Column :: HardLinks) ; } if self . filesize { columns . push (Column :: FileSize) ; } if self . blocksize { # [cfg (unix)] columns . push (Column :: Blocksize) ; } if self . user { # [cfg (unix)] columns . push (Column :: User) ; } if self . group { # [cfg (unix)] columns . push (Column :: Group) ; } if self . file_flags { columns . push (Column :: FileFlags) ; } # [cfg (target_os = "linux")] if self . security_context { columns . push (Column :: SecurityContext) ; } if self . time_types . modified { columns . push (Column :: Timestamp (TimeType :: Modified)) ; } if self . time_types . changed { columns . push (Column :: Timestamp (TimeType :: Changed)) ; } if self . time_types . created { columns . push (Column :: Timestamp (TimeType :: Created)) ; } if self . time_types . accessed { columns . push (Column :: Timestamp (TimeType :: Accessed)) ; } if self . git && actually_enable_git { columns . push (Column :: GitStatus) ; } if self . subdir_git_repos && git_repos { columns . push (Column :: SubdirGitRepo (true)) ; } if self . subdir_git_repos_no_stat && git_repos { columns . push (Column :: SubdirGitRepo (false)) ; } columns } }
};
}
