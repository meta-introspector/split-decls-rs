mkuse!{use rustc_span :: { Symbol , sym } ;}
mkuse!{pub use self :: hcx :: StableHashingContext ;}
mkmod!{hcx, { 
                getname!(hcx);
                getsrc!(hcx);
                getpath!(hcx);
                get_deps!(hcx);
                get_crates!(hcx);
                mkinclude!(hcx);
                 
            }}
mkmod!{impls_syntax, { 
                getname!(impls_syntax);
                getsrc!(impls_syntax);
                getpath!(impls_syntax);
                get_deps!(impls_syntax);
                get_crates!(impls_syntax);
                mkinclude!(impls_syntax);
                 
            }}
mkitem!{pub const IGNORED_ATTRIBUTES : & [Symbol] = & [sym :: cfg_trace , sym :: rustc_if_this_changed , sym :: rustc_then_this_would_need , sym :: rustc_dirty , sym :: rustc_clean , sym :: rustc_partition_reused , sym :: rustc_partition_codegened , sym :: rustc_expected_cgu_reuse ,] ;}