mkmod!{libm_helper, { 
                getname!(libm_helper);
                getsrc!(libm_helper);
                getpath!(libm_helper);
                get_deps!(libm_helper);
                get_crates!(libm_helper);
                mkinclude!(libm_helper);
                 
            }}
mkmod!{math, { 
                getname!(math);
                getsrc!(math);
                getpath!(math);
                get_deps!(math);
                get_crates!(math);
                mkinclude!(math);
                 
            }}
mkuse!{use core :: { f32 , f64 } ;}
mkuse!{pub use libm_helper :: * ;}
mkuse!{pub use self :: math :: * ;}