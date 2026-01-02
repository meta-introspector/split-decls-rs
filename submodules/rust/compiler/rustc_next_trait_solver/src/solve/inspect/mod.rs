mkuse!{pub use rustc_type_ir :: solve :: inspect :: * ;}
mkmod!{build, { 
                getname!(build);
                getsrc!(build);
                getpath!(build);
                get_deps!(build);
                get_crates!(build);
                mkinclude!(build);
                 
            }}
mkuse!{pub (in crate :: solve) use build :: * ;}
mkuse!{pub use crate :: solve :: eval_ctxt :: canonical :: instantiate_canonical_state ;}