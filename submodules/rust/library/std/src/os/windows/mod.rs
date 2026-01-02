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
mkuse!{# [doc (no_inline)] # [stable (feature = "file_offset" , since = "1.15.0")] pub use super :: fs :: FileExt ;}
mkuse!{# [doc (no_inline)] # [stable (feature = "rust1" , since = "1.0.0")] pub use super :: fs :: { MetadataExt , OpenOptionsExt } ;}
mkuse!{# [doc (no_inline)] # [stable (feature = "rust1" , since = "1.0.0")] pub use super :: io :: { AsHandle , AsSocket , BorrowedHandle , BorrowedSocket , FromRawHandle , FromRawSocket , HandleOrInvalid , IntoRawHandle , IntoRawSocket , OwnedHandle , OwnedSocket , } ;}
mkuse!{# [doc (no_inline)] # [stable (feature = "rust1" , since = "1.0.0")] pub use super :: io :: { AsRawHandle , AsRawSocket , RawHandle , RawSocket } ;} 
            }}