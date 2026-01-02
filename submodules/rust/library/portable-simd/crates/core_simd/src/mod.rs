mkmod!{swizzle, { 
                getname!(swizzle);
                getsrc!(swizzle);
                getpath!(swizzle);
                get_deps!(swizzle);
                get_crates!(swizzle);
                mkinclude!(swizzle);
                 
            }}
mkmod!{alias, { 
                getname!(alias);
                getsrc!(alias);
                getpath!(alias);
                get_deps!(alias);
                get_crates!(alias);
                mkinclude!(alias);
                 
            }}
mkmod!{cast, { 
                getname!(cast);
                getsrc!(cast);
                getpath!(cast);
                get_deps!(cast);
                get_crates!(cast);
                mkinclude!(cast);
                 
            }}
mkmod!{fmt, { 
                getname!(fmt);
                getsrc!(fmt);
                getpath!(fmt);
                get_deps!(fmt);
                get_crates!(fmt);
                mkinclude!(fmt);
                 
            }}
mkmod!{iter, { 
                getname!(iter);
                getsrc!(iter);
                getpath!(iter);
                get_deps!(iter);
                get_crates!(iter);
                mkinclude!(iter);
                 
            }}
mkmod!{lane_count, { 
                getname!(lane_count);
                getsrc!(lane_count);
                getpath!(lane_count);
                get_deps!(lane_count);
                get_crates!(lane_count);
                mkinclude!(lane_count);
                 
            }}
mkmod!{masks, { 
                getname!(masks);
                getsrc!(masks);
                getpath!(masks);
                get_deps!(masks);
                get_crates!(masks);
                mkinclude!(masks);
                 
            }}
mkmod!{ops, { 
                getname!(ops);
                getsrc!(ops);
                getpath!(ops);
                get_deps!(ops);
                get_crates!(ops);
                mkinclude!(ops);
                 
            }}
mkmod!{select, { 
                getname!(select);
                getsrc!(select);
                getpath!(select);
                get_deps!(select);
                get_crates!(select);
                mkinclude!(select);
                 
            }}
mkmod!{swizzle_dyn, { 
                getname!(swizzle_dyn);
                getsrc!(swizzle_dyn);
                getpath!(swizzle_dyn);
                get_deps!(swizzle_dyn);
                get_crates!(swizzle_dyn);
                mkinclude!(swizzle_dyn);
                 
            }}
mkmod!{to_bytes, { 
                getname!(to_bytes);
                getsrc!(to_bytes);
                getpath!(to_bytes);
                get_deps!(to_bytes);
                get_crates!(to_bytes);
                mkinclude!(to_bytes);
                 
            }}
mkmod!{vector, { 
                getname!(vector);
                getsrc!(vector);
                getpath!(vector);
                get_deps!(vector);
                get_crates!(vector);
                mkinclude!(vector);
                 
            }}
mkmod!{vendor, { 
                getname!(vendor);
                getsrc!(vendor);
                getpath!(vendor);
                get_deps!(vendor);
                get_crates!(vendor);
                mkinclude!(vendor);
                 
            }}
mkmod!{simd, { 
                getname!(simd);
                getsrc!(simd);
                getpath!(simd);
                get_deps!(simd);
                get_crates!(simd);
                mkinclude!(simd);
                mkmod!{prelude, { 
                getname!(prelude);
                getsrc!(prelude);
                getpath!(prelude);
                get_deps!(prelude);
                get_crates!(prelude);
                mkinclude!(prelude);
                 
            }}
mkmod!{num, { 
                getname!(num);
                getsrc!(num);
                getpath!(num);
                get_deps!(num);
                get_crates!(num);
                mkinclude!(num);
                 
            }}
mkmod!{ptr, { 
                getname!(ptr);
                getsrc!(ptr);
                getpath!(ptr);
                get_deps!(ptr);
                get_crates!(ptr);
                mkinclude!(ptr);
                 
            }}
mkmod!{cmp, { 
                getname!(cmp);
                getsrc!(cmp);
                getpath!(cmp);
                get_deps!(cmp);
                get_crates!(cmp);
                mkinclude!(cmp);
                 
            }}
mkuse!{pub use crate :: core_simd :: alias :: * ;}
mkuse!{pub use crate :: core_simd :: cast :: * ;}
mkuse!{pub use crate :: core_simd :: lane_count :: { LaneCount , SupportedLaneCount } ;}
mkuse!{pub use crate :: core_simd :: masks :: * ;}
mkuse!{pub use crate :: core_simd :: swizzle :: * ;}
mkuse!{pub use crate :: core_simd :: to_bytes :: ToBytes ;}
mkuse!{pub use crate :: core_simd :: vector :: * ;} 
            }}