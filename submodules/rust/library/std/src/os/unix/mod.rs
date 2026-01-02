mkuse!{# [cfg (doc)] use crate :: os :: linux as platform ;}
mkmod!{platform, { 
                getname!(platform);
                getsrc!(platform);
                getpath!(platform);
                get_deps!(platform);
                get_crates!(platform);
                mkinclude!(platform);
                mkuse!{# [cfg (target_os = "aix")] pub use crate :: os :: aix :: * ;}
mkuse!{# [cfg (target_os = "android")] pub use crate :: os :: android :: * ;}
mkuse!{# [cfg (target_os = "cygwin")] pub use crate :: os :: cygwin :: * ;}
mkuse!{# [cfg (target_vendor = "apple")] pub use crate :: os :: darwin :: * ;}
mkuse!{# [cfg (target_os = "dragonfly")] pub use crate :: os :: dragonfly :: * ;}
mkuse!{# [cfg (target_os = "emscripten")] pub use crate :: os :: emscripten :: * ;}
mkuse!{# [cfg (target_os = "espidf")] pub use crate :: os :: espidf :: * ;}
mkuse!{# [cfg (target_os = "freebsd")] pub use crate :: os :: freebsd :: * ;}
mkuse!{# [cfg (target_os = "fuchsia")] pub use crate :: os :: fuchsia :: * ;}
mkuse!{# [cfg (target_os = "haiku")] pub use crate :: os :: haiku :: * ;}
mkuse!{# [cfg (target_os = "horizon")] pub use crate :: os :: horizon :: * ;}
mkuse!{# [cfg (target_os = "hurd")] pub use crate :: os :: hurd :: * ;}
mkuse!{# [cfg (target_os = "illumos")] pub use crate :: os :: illumos :: * ;}
mkuse!{# [cfg (target_os = "l4re")] pub use crate :: os :: l4re :: * ;}
mkuse!{# [cfg (target_os = "linux")] pub use crate :: os :: linux :: * ;}
mkuse!{# [cfg (target_os = "netbsd")] pub use crate :: os :: netbsd :: * ;}
mkuse!{# [cfg (target_os = "nto")] pub use crate :: os :: nto :: * ;}
mkuse!{# [cfg (target_os = "nuttx")] pub use crate :: os :: nuttx :: * ;}
mkuse!{# [cfg (target_os = "openbsd")] pub use crate :: os :: openbsd :: * ;}
mkuse!{# [cfg (target_os = "redox")] pub use crate :: os :: redox :: * ;}
mkuse!{# [cfg (target_os = "rtems")] pub use crate :: os :: rtems :: * ;}
mkuse!{# [cfg (target_os = "solaris")] pub use crate :: os :: solaris :: * ;}
mkuse!{# [cfg (target_os = "vita")] pub use crate :: os :: vita :: * ;}
mkuse!{# [cfg (target_os = "vxworks")] pub use crate :: os :: vxworks :: * ;} 
            }}
mkmod!{ffi, { 
                getname!(ffi);
                getsrc!(ffi);
                getpath!(ffi);
                get_deps!(ffi);
                get_crates!(ffi);
                mkinclude!(ffi);
                 
            }}
mkmod!{fs, { 
                getname!(fs);
                getsrc!(fs);
                getpath!(fs);
                get_deps!(fs);
                get_crates!(fs);
                mkinclude!(fs);
                 
            }}
mkmod!{io, { 
                getname!(io);
                getsrc!(io);
                getpath!(io);
                get_deps!(io);
                get_crates!(io);
                mkinclude!(io);
                 
            }}
mkmod!{net, { 
                getname!(net);
                getsrc!(net);
                getpath!(net);
                get_deps!(net);
                get_crates!(net);
                mkinclude!(net);
                 
            }}
mkmod!{process, { 
                getname!(process);
                getsrc!(process);
                getpath!(process);
                get_deps!(process);
                get_crates!(process);
                mkinclude!(process);
                 
            }}
mkmod!{raw, { 
                getname!(raw);
                getsrc!(raw);
                getpath!(raw);
                get_deps!(raw);
                get_crates!(raw);
                mkinclude!(raw);
                 
            }}
mkmod!{thread, { 
                getname!(thread);
                getsrc!(thread);
                getpath!(thread);
                get_deps!(thread);
                get_crates!(thread);
                mkinclude!(thread);
                 
            }}
mkmod!{prelude, { 
                getname!(prelude);
                getsrc!(prelude);
                getpath!(prelude);
                get_deps!(prelude);
                get_crates!(prelude);
                mkinclude!(prelude);
                mkuse!{# [doc (no_inline)] # [stable (feature = "rust1" , since = "1.0.0")] pub use super :: ffi :: { OsStrExt , OsStringExt } ;}
mkuse!{# [doc (no_inline)] # [stable (feature = "rust1" , since = "1.0.0")] pub use super :: fs :: DirEntryExt ;}
mkuse!{# [doc (no_inline)] # [stable (feature = "file_offset" , since = "1.15.0")] pub use super :: fs :: FileExt ;}
mkuse!{# [doc (no_inline)] # [stable (feature = "rust1" , since = "1.0.0")] pub use super :: fs :: { FileTypeExt , MetadataExt , OpenOptionsExt , PermissionsExt } ;}
mkuse!{# [doc (no_inline)] # [stable (feature = "rust1" , since = "1.0.0")] pub use super :: io :: { AsFd , AsRawFd , BorrowedFd , FromRawFd , IntoRawFd , OwnedFd , RawFd } ;}
mkuse!{# [doc (no_inline)] # [unstable (feature = "unix_send_signal" , issue = "141975")] pub use super :: process :: ChildExt ;}
mkuse!{# [doc (no_inline)] # [stable (feature = "rust1" , since = "1.0.0")] pub use super :: process :: { CommandExt , ExitStatusExt } ;}
mkuse!{# [doc (no_inline)] # [stable (feature = "rust1" , since = "1.0.0")] pub use super :: thread :: JoinHandleExt ;} 
            }}