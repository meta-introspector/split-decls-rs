mkuse!{use crate :: os :: raw :: c_ulong ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type dev_t = u64 ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type mode_t = u32 ;}
mkitem!{# [stable (feature = "pthread_t" , since = "1.8.0")] pub type pthread_t = c_ulong ;}
mkuse!{# [doc (inline)] # [stable (feature = "raw_ext" , since = "1.1.0")] pub use self :: arch :: { blkcnt_t , blksize_t , ino_t , nlink_t , off_t , stat , time_t } ;}
mkmod!{arch, { 
                getname!(arch);
                getsrc!(arch);
                getpath!(arch);
                get_deps!(arch);
                get_crates!(arch);
                mkinclude!(arch);
                mkuse!{use crate :: os :: raw :: { c_long , c_short , c_uint } ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type blkcnt_t = u64 ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type blksize_t = u64 ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type ino_t = u64 ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type nlink_t = u64 ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type off_t = u64 ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type time_t = i64 ;}
mkitem!{mkstruct!{# [repr (C)] # [derive (Clone)] # [stable (feature = "raw_ext" , since = "1.1.0")] pub struct stat { # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_dev : u64 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub __pad1 : c_short , # [stable (feature = "raw_ext" , since = "1.1.0")] pub __st_ino : u32 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_mode : u32 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_nlink : u32 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_uid : u32 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_gid : u32 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_rdev : u64 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub __pad2 : c_uint , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_size : i64 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_blksize : i32 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_blocks : i64 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_atime : i32 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_atime_nsec : c_long , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_mtime : i32 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_mtime_nsec : c_long , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_ctime : i32 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_ctime_nsec : c_long , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_ino : u64 , }}} 
            }}
mkmod!{arch, { 
                getname!(arch);
                getsrc!(arch);
                getpath!(arch);
                get_deps!(arch);
                get_crates!(arch);
                mkinclude!(arch);
                mkuse!{use crate :: os :: raw :: { c_long , c_ulong } ;}
mkitem!{# [cfg (target_env = "musl")] # [stable (feature = "raw_ext" , since = "1.1.0")] pub type blkcnt_t = i64 ;}
mkitem!{# [cfg (not (target_env = "musl"))] # [stable (feature = "raw_ext" , since = "1.1.0")] pub type blkcnt_t = u64 ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type blksize_t = u64 ;}
mkitem!{# [cfg (target_env = "musl")] # [stable (feature = "raw_ext" , since = "1.1.0")] pub type ino_t = u64 ;}
mkitem!{# [cfg (not (target_env = "musl"))] # [stable (feature = "raw_ext" , since = "1.1.0")] pub type ino_t = u64 ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type nlink_t = u64 ;}
mkitem!{# [cfg (target_env = "musl")] # [stable (feature = "raw_ext" , since = "1.1.0")] pub type off_t = u64 ;}
mkitem!{# [cfg (not (target_env = "musl"))] # [stable (feature = "raw_ext" , since = "1.1.0")] pub type off_t = u64 ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type time_t = i64 ;}
mkitem!{mkstruct!{# [repr (C)] # [derive (Clone)] # [stable (feature = "raw_ext" , since = "1.1.0")] pub struct stat { # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_dev : c_ulong , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_pad1 : [c_long ; 3] , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_ino : u64 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_mode : u32 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_nlink : u32 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_uid : u32 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_gid : u32 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_rdev : c_ulong , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_pad2 : [c_long ; 2] , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_size : i64 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_atime : i32 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_atime_nsec : c_long , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_mtime : i32 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_mtime_nsec : c_long , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_ctime : i32 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_ctime_nsec : c_long , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_blksize : i32 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_blocks : i64 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_pad5 : [c_long ; 14] , }}} 
            }}
mkmod!{arch, { 
                getname!(arch);
                getsrc!(arch);
                getpath!(arch);
                get_deps!(arch);
                get_crates!(arch);
                mkinclude!(arch);
                mkuse!{use crate :: os :: raw :: { c_int , c_long , c_uint } ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type blkcnt_t = i64 ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type blksize_t = c_long ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type ino_t = u64 ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type nlink_t = c_uint ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type off_t = i64 ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type time_t = i64 ;}
mkitem!{mkstruct!{# [repr (C)] # [derive (Clone)] # [stable (feature = "raw_ext" , since = "1.1.0")] pub struct stat { # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_dev : u64 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_ino : u64 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_mode : u32 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_nlink : u32 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_uid : u32 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_gid : u32 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_rdev : u64 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub __pad1 : u32 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_size : i64 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_blksize : i32 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub __pad2 : i32 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_blocks : i64 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_atime : i64 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_atime_nsec : c_long , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_mtime : i64 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_mtime_nsec : c_long , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_ctime : i64 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_ctime_nsec : c_long , # [stable (feature = "raw_ext" , since = "1.1.0")] pub __pad3 : [c_int ; 2] , }}} 
            }}
mkmod!{arch, { 
                getname!(arch);
                getsrc!(arch);
                getpath!(arch);
                get_deps!(arch);
                get_crates!(arch);
                mkinclude!(arch);
                mkuse!{pub use libc :: { blkcnt_t , blksize_t , ino_t , nlink_t , off_t , stat , time_t } ;} 
            }}
mkmod!{arch, { 
                getname!(arch);
                getsrc!(arch);
                getpath!(arch);
                get_deps!(arch);
                get_crates!(arch);
                mkinclude!(arch);
                mkuse!{use crate :: os :: raw :: { c_int , c_long } ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type blkcnt_t = i64 ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type blksize_t = i32 ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type ino_t = u64 ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type nlink_t = u32 ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type off_t = i64 ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type time_t = c_long ;}
mkitem!{mkstruct!{# [repr (C)] # [derive (Clone)] # [stable (feature = "raw_ext" , since = "1.1.0")] pub struct stat { # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_dev : u64 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_ino : u64 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_mode : u32 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_nlink : u32 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_uid : u32 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_gid : u32 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_rdev : u64 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub __pad1 : u64 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_size : i64 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_blksize : i32 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub __pad2 : c_int , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_blocks : i64 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_atime : time_t , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_atime_nsec : c_long , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_mtime : time_t , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_mtime_nsec : c_long , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_ctime : time_t , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_ctime_nsec : c_long , # [stable (feature = "raw_ext" , since = "1.1.0")] pub __unused : [c_int ; 2] , }}} 
            }}
mkmod!{arch, { 
                getname!(arch);
                getsrc!(arch);
                getpath!(arch);
                get_deps!(arch);
                get_crates!(arch);
                mkinclude!(arch);
                mkuse!{use crate :: os :: raw :: { c_int , c_long } ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type blkcnt_t = u64 ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type blksize_t = u64 ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type ino_t = u64 ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type nlink_t = u64 ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type off_t = u64 ;}
mkitem!{# [stable (feature = "raw_ext" , since = "1.1.0")] pub type time_t = i64 ;}
mkitem!{mkstruct!{# [repr (C)] # [derive (Clone)] # [stable (feature = "raw_ext" , since = "1.1.0")] pub struct stat { # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_dev : u64 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_ino : u64 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_nlink : u64 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_mode : u32 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_uid : u32 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_gid : u32 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub __pad0 : c_int , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_rdev : u64 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_size : i64 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_blksize : i64 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_blocks : i64 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_atime : i64 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_atime_nsec : c_long , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_mtime : i64 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_mtime_nsec : c_long , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_ctime : i64 , # [stable (feature = "raw_ext" , since = "1.1.0")] pub st_ctime_nsec : c_long , # [stable (feature = "raw_ext" , since = "1.1.0")] pub __unused : [c_long ; 3] , }}} 
            }}