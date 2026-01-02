mkitem!{# [cfg (test)] # [macro_use] extern crate std ;}
mkitem!{# [cfg_attr (feature = "std_detect_file_io" , allow (unused_extern_crates))] # [cfg (feature = "std_detect_file_io")] extern crate alloc ;}
mkmod!{detect, { 
                getname!(detect);
                getsrc!(detect);
                getpath!(detect);
                get_deps!(detect);
                get_crates!(detect);
                mkinclude!(detect);
                 
            }}