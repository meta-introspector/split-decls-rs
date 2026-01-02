mkitem!{extern crate alloc ;}
mkmod!{__internal, { 
                getname!(__internal);
                getsrc!(__internal);
                getpath!(__internal);
                get_deps!(__internal);
                get_crates!(__internal);
                mkinclude!(__internal);
                mkitem!{extern crate std ;}
mkuse!{pub use std :: * ;} 
            }}
mkuse!{# [allow (unused_imports)] use __internal :: * ;}
mkmod!{the_backtrace_crate, { 
                getname!(the_backtrace_crate);
                getsrc!(the_backtrace_crate);
                getpath!(the_backtrace_crate);
                get_deps!(the_backtrace_crate);
                get_crates!(the_backtrace_crate);
                mkinclude!(the_backtrace_crate);
                 
            }}