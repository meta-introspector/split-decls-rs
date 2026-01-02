mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] # [allow (non_camel_case_types)] pub type uid_t = u32 ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] # [allow (non_camel_case_types)] pub type gid_t = u32 ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] # [allow (non_camel_case_types)] pub type pid_t = i32 ;}
mkuse!{# [doc (inline)] # [stable (feature = "pthread_t" , since = "1.8.0")] pub use super :: platform :: raw :: pthread_t ;}
mkuse!{# [doc (inline)] # [stable (feature = "raw_ext" , since = "1.1.0")] pub use super :: platform :: raw :: { blkcnt_t , time_t } ;}
mkuse!{# [doc (inline)] # [stable (feature = "raw_ext" , since = "1.1.0")] pub use super :: platform :: raw :: { blksize_t , dev_t , ino_t , mode_t , nlink_t , off_t } ;}