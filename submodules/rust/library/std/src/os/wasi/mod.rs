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
mkmod!{prelude, { 
                getname!(prelude);
                getsrc!(prelude);
                getpath!(prelude);
                get_deps!(prelude);
                get_crates!(prelude);
                mkinclude!(prelude);
                mkuse!{# [doc (no_inline)] # [stable (feature = "rust1" , since = "1.0.0")] pub use super :: ffi :: { OsStrExt , OsStringExt } ;}
mkuse!{# [doc (no_inline)] # [stable (feature = "rust1" , since = "1.0.0")] pub use super :: fs :: FileTypeExt ;}
mkuse!{# [doc (no_inline)] # [stable (feature = "rust1" , since = "1.0.0")] pub use super :: fs :: { DirEntryExt , FileExt , MetadataExt , OpenOptionsExt } ;}
mkuse!{# [doc (no_inline)] # [stable (feature = "rust1" , since = "1.0.0")] pub use super :: io :: { AsFd , AsRawFd , BorrowedFd , FromRawFd , IntoRawFd , OwnedFd , RawFd } ;} 
            }}