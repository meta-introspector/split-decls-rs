mkmod!{specialized_div_rem, { 
                getname!(specialized_div_rem);
                getsrc!(specialized_div_rem);
                getpath!(specialized_div_rem);
                get_deps!(specialized_div_rem);
                get_crates!(specialized_div_rem);
                mkinclude!(specialized_div_rem);
                 
            }}
mkmod!{addsub, { 
                getname!(addsub);
                getsrc!(addsub);
                getpath!(addsub);
                get_deps!(addsub);
                get_crates!(addsub);
                mkinclude!(addsub);
                 
            }}
mkmod!{big, { 
                getname!(big);
                getsrc!(big);
                getpath!(big);
                get_deps!(big);
                get_crates!(big);
                mkinclude!(big);
                 
            }}
mkmod!{bswap, { 
                getname!(bswap);
                getsrc!(bswap);
                getpath!(bswap);
                get_deps!(bswap);
                get_crates!(bswap);
                mkinclude!(bswap);
                 
            }}
mkmod!{leading_zeros, { 
                getname!(leading_zeros);
                getsrc!(leading_zeros);
                getpath!(leading_zeros);
                get_deps!(leading_zeros);
                get_crates!(leading_zeros);
                mkinclude!(leading_zeros);
                 
            }}
mkmod!{mul, { 
                getname!(mul);
                getsrc!(mul);
                getpath!(mul);
                get_deps!(mul);
                get_crates!(mul);
                mkinclude!(mul);
                 
            }}
mkmod!{sdiv, { 
                getname!(sdiv);
                getsrc!(sdiv);
                getpath!(sdiv);
                get_deps!(sdiv);
                get_crates!(sdiv);
                mkinclude!(sdiv);
                 
            }}
mkmod!{shift, { 
                getname!(shift);
                getsrc!(shift);
                getpath!(shift);
                get_deps!(shift);
                get_crates!(shift);
                mkinclude!(shift);
                 
            }}
mkmod!{trailing_zeros, { 
                getname!(trailing_zeros);
                getsrc!(trailing_zeros);
                getpath!(trailing_zeros);
                get_deps!(trailing_zeros);
                get_crates!(trailing_zeros);
                mkinclude!(trailing_zeros);
                 
            }}
mkmod!{traits, { 
                getname!(traits);
                getsrc!(traits);
                getpath!(traits);
                get_deps!(traits);
                get_crates!(traits);
                mkinclude!(traits);
                 
            }}
mkmod!{udiv, { 
                getname!(udiv);
                getsrc!(udiv);
                getpath!(udiv);
                get_deps!(udiv);
                get_crates!(udiv);
                mkinclude!(udiv);
                 
            }}
mkuse!{pub use big :: { i256 , u256 } ;}
mkuse!{# [cfg (not (feature = "unstable-public-internals"))] pub (crate) use traits :: { CastFrom , CastInto , DInt , HInt , Int , MinInt } ;}
mkuse!{# [cfg (feature = "unstable-public-internals")] pub use traits :: { CastFrom , CastInto , DInt , HInt , Int , MinInt } ;}