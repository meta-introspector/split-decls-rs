mkuse!{pub use rustc_middle :: ty :: relate :: combine :: PredicateEmittingRelation ;}
mkuse!{pub use rustc_middle :: ty :: relate :: { RelateResult , * } ;}
mkmod!{generalize, { 
                getname!(generalize);
                getsrc!(generalize);
                getpath!(generalize);
                get_deps!(generalize);
                get_crates!(generalize);
                mkinclude!(generalize);
                 
            }}
mkmod!{higher_ranked, { 
                getname!(higher_ranked);
                getsrc!(higher_ranked);
                getpath!(higher_ranked);
                get_deps!(higher_ranked);
                get_crates!(higher_ranked);
                mkinclude!(higher_ranked);
                 
            }}
mkmod!{lattice, { 
                getname!(lattice);
                getsrc!(lattice);
                getpath!(lattice);
                get_deps!(lattice);
                get_crates!(lattice);
                mkinclude!(lattice);
                 
            }}
mkmod!{type_relating, { 
                getname!(type_relating);
                getsrc!(type_relating);
                getpath!(type_relating);
                get_deps!(type_relating);
                get_crates!(type_relating);
                mkinclude!(type_relating);
                 
            }}