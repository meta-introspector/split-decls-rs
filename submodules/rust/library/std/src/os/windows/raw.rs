mkuse!{use crate :: os :: raw :: c_void ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type HANDLE = * mut c_void ;}
mkitem!{# [cfg (target_pointer_width = "32")] # [doc (cfg (all ()))] # [stable (feature = "raw_ext" , since = "1.1.0")] pub type SOCKET = u32 ;}
mkitem!{# [cfg (target_pointer_width = "64")] # [doc (cfg (all ()))] # [stable (feature = "raw_ext" , since = "1.1.0")] pub type SOCKET = u64 ;}