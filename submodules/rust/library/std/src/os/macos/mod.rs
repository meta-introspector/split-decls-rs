mkmod!{fs, { 
                getname!(fs);
                getsrc!(fs);
                getpath!(fs);
                get_deps!(fs);
                get_crates!(fs);
                mkinclude!(fs);
                mkuse!{# [stable (feature = "file_set_times" , since = "1.75.0")] pub use crate :: os :: darwin :: fs :: FileTimesExt ;}
mkuse!{# [stable (feature = "metadata_ext" , since = "1.1.0")] pub use crate :: os :: darwin :: fs :: MetadataExt ;} 
            }}
mkmod!{raw, { 
                getname!(raw);
                getsrc!(raw);
                getpath!(raw);
                get_deps!(raw);
                get_crates!(raw);
                mkinclude!(raw);
                mkuse!{# [doc (inline)] # [stable (feature = "raw_ext" , since = "1.1.0")] pub use crate :: os :: darwin :: raw :: * ;} 
            }}