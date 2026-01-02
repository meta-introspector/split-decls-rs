mkmod!{os_str, { 
                getname!(os_str);
                getsrc!(os_str);
                getpath!(os_str);
                get_deps!(os_str);
                get_crates!(os_str);
                mkinclude!(os_str);
                 
            }}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use self :: os_str :: { OsStrExt , OsStringExt } ;}