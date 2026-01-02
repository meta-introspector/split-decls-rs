mkuse!{use crate :: os :: raw :: c_long ;}
mkitem!{# [stable (feature = "pthread_t" , since = "1.8.0")] pub type pthread_t = c_long ;}
mkuse!{# [doc (inline)] # [stable (feature = "raw_ext" , since = "1.1.0")] pub use self :: arch :: { blkcnt_t , blksize_t , dev_t , ino_t , mode_t , nlink_t , off_t , stat , time_t } ;}
mkmod!{arch, { 
                getname!(arch);
                getsrc!(arch);
                getpath!(arch);
                get_deps!(arch);
                get_crates!(arch);
                mkinclude!(arch);
                mkuse!{use crate :: os :: raw :: { c_long , c_longlong , c_uchar , c_uint , c_ulong , c_ulonglong } ;}
mkuse!{use crate :: os :: unix :: raw :: { gid_t , uid_t } ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type dev_t = u32 ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type mode_t = c_uint ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type blkcnt_t = c_ulong ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type blksize_t = c_ulong ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type ino_t = c_ulong ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type nlink_t = u32 ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type off_t = i32 ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type time_t = c_long ;}
mkitem!{mkstruct!{# [repr (C)] # [derive (Clone)] # [stable (feature = "raw_ext" , since = "1.1.0")] pub struct stat { # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_dev : c_ulonglong , # [stable (feature = "raw_ext" , since = "1.1.0")] pub __pad0 : [c_uchar ; 4] , # [stable (feature = "raw_ext" , since = "1.1.0")] pub __st_ino : u32 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_mode : c_uint , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_nlink : c_uint , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_uid : uid_t , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_gid : gid_t , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_rdev : c_ulonglong , # [stable (feature = "raw_ext" , since = "1.1.0")] pub __pad3 : [c_uchar ; 4] , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_size : c_longlong , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_blksize : u32 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_blocks : c_ulonglong , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_atime : time_t , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_atime_nsec : c_long , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_mtime : time_t , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_mtime_nsec : c_long , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_ctime : time_t , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_ctime_nsec : c_long , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_ino : c_ulonglong , }}} 
            }}
mkmod!{arch, { 
                getname!(arch);
                getsrc!(arch);
                getpath!(arch);
                get_deps!(arch);
                get_crates!(arch);
                mkinclude!(arch);
                mkuse!{use crate :: os :: raw :: { c_int , c_long , c_uint , c_ulong } ;}
mkuse!{use crate :: os :: unix :: raw :: { gid_t , uid_t } ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type dev_t = u64 ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type mode_t = c_uint ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type blkcnt_t = c_ulong ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type blksize_t = c_ulong ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type ino_t = c_ulong ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type nlink_t = u32 ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type off_t = i64 ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type time_t = c_long ;}
mkitem!{mkstruct!{# [repr (C)] # [derive (Clone)] # [stable (feature = "raw_ext" , since = "1.1.0")] pub struct stat { # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_dev : dev_t , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_ino : ino_t , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_mode : mode_t , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_nlink : nlink_t , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_uid : uid_t , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_gid : gid_t , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_rdev : dev_t , # [stable (feature = "raw_ext" , since = "1.1.0")] pub __pad1 : c_ulong , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_size : off_t , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_blksize : c_int , # [stable (feature = "raw_ext" , since = "1.1.0")] pub __pad2 : c_int , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_blocks : c_long , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_atime : time_t , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_atime_nsec : c_long , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_mtime : time_t , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_mtime_nsec : c_long , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_ctime : time_t , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_ctime_nsec : c_long , # [stable (feature = "raw_ext" , since = "1.1.0")] pub __unused4 : c_uint , # [stable (feature = "raw_ext" , since = "1.1.0")] pub __unused5 : c_uint , }}} 
            }}
mkmod!{arch, { 
                getname!(arch);
                getsrc!(arch);
                getpath!(arch);
                get_deps!(arch);
                get_crates!(arch);
                mkinclude!(arch);
                mkuse!{use crate :: os :: raw :: { c_long , c_uint , c_ulong } ;}
mkuse!{use crate :: os :: unix :: raw :: { gid_t , uid_t } ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type dev_t = u64 ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type mode_t = c_uint ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type blkcnt_t = c_ulong ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type blksize_t = c_ulong ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type ino_t = c_ulong ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type nlink_t = u32 ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type off_t = i64 ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type time_t = c_long ;}
mkitem!{mkstruct!{# [repr (C)] # [derive (Clone)] # [stable (feature = "raw_ext" , since = "1.1.0")] pub struct stat { # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_dev : dev_t , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_ino : ino_t , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_nlink : c_ulong , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_mode : c_uint , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_uid : uid_t , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_gid : gid_t , # [stable (feature = "raw_ext" , since = "1.1.0")] pub __pad0 : c_uint , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_rdev : dev_t , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_size : off_t , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_blksize : c_long , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_blocks : c_long , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_atime : time_t , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_atime_nsec : c_long , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_mtime : time_t , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_mtime_nsec : c_long , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_ctime : time_t , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_ctime_nsec : c_long , # [stable (feature = "raw_ext" , since = "1.1.0")] pub __pad3 : [c_long ; 3] , }}} 
            }}