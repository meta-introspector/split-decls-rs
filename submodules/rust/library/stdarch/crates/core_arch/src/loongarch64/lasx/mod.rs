mkmod!{types, { 
                getname!(types);
                getsrc!(types);
                getpath!(types);
                get_deps!(types);
                get_crates!(types);
                mkinclude!(types);
                 
            }}
mkuse!{# [rustfmt :: skip] # [unstable (feature = "stdarch_loongarch" , issue = "117427")] pub use self :: types :: * ;}
mkmod!{generated, { 
                getname!(generated);
                getsrc!(generated);
                getpath!(generated);
                get_deps!(generated);
                get_crates!(generated);
                mkinclude!(generated);
                 
            }}
mkuse!{# [rustfmt :: skip] # [unstable (feature = "stdarch_loongarch" , issue = "117427")] pub use self :: generated :: * ;}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}