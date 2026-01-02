mkuse!{use crate :: os :: raw :: c_int ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type dev_t = u32 ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type mode_t = u32 ;}
mkitem!{# [stable (feature = "pthread_t" , since = "1.8.0")] pub type pthread_t = c_int ;}
mkuse!{# [doc (inline)] # [stable (feature = "raw_ext" , since = "1.1.0")] pub use self :: arch :: { blkcnt_t , blksize_t , ino_t , nlink_t , off_t , time_t } ;}
mkmod!{arch, { 
                getname!(arch);
                getsrc!(arch);
                getpath!(arch);
                get_deps!(arch);
                get_crates!(arch);
                mkinclude!(arch);
                mkuse!{use crate :: os :: raw :: c_long ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type blkcnt_t = i64 ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type blksize_t = i32 ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type ino_t = u64 ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type nlink_t = u32 ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type off_t = i64 ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type time_t = c_long ;} 
            }}