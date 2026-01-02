mkmod!{dropck_outlives, { 
                getname!(dropck_outlives);
                getsrc!(dropck_outlives);
                getpath!(dropck_outlives);
                get_deps!(dropck_outlives);
                get_crates!(dropck_outlives);
                mkinclude!(dropck_outlives);
                 
            }}
mkmod!{evaluate_obligation, { 
                getname!(evaluate_obligation);
                getsrc!(evaluate_obligation);
                getpath!(evaluate_obligation);
                get_deps!(evaluate_obligation);
                get_crates!(evaluate_obligation);
                mkinclude!(evaluate_obligation);
                 
            }}
mkmod!{method_autoderef, { 
                getname!(method_autoderef);
                getsrc!(method_autoderef);
                getpath!(method_autoderef);
                get_deps!(method_autoderef);
                get_crates!(method_autoderef);
                mkinclude!(method_autoderef);
                 
            }}
mkmod!{normalize, { 
                getname!(normalize);
                getsrc!(normalize);
                getpath!(normalize);
                get_deps!(normalize);
                get_crates!(normalize);
                mkinclude!(normalize);
                 
            }}
mkmod!{type_op, { 
                getname!(type_op);
                getsrc!(type_op);
                getpath!(type_op);
                get_deps!(type_op);
                get_crates!(type_op);
                mkinclude!(type_op);
                 
            }}
mkuse!{pub use rustc_middle :: traits :: query :: * ;}