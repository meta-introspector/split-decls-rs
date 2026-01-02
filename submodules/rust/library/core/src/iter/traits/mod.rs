mkmod!{accum, { 
                getname!(accum);
                getsrc!(accum);
                getpath!(accum);
                get_deps!(accum);
                get_crates!(accum);
                mkinclude!(accum);
                 
            }}
mkmod!{collect, { 
                getname!(collect);
                getsrc!(collect);
                getpath!(collect);
                get_deps!(collect);
                get_crates!(collect);
                mkinclude!(collect);
                 
            }}
mkmod!{double_ended, { 
                getname!(double_ended);
                getsrc!(double_ended);
                getpath!(double_ended);
                get_deps!(double_ended);
                get_crates!(double_ended);
                mkinclude!(double_ended);
                 
            }}
mkmod!{exact_size, { 
                getname!(exact_size);
                getsrc!(exact_size);
                getpath!(exact_size);
                get_deps!(exact_size);
                get_crates!(exact_size);
                mkinclude!(exact_size);
                 
            }}
mkmod!{iterator, { 
                getname!(iterator);
                getsrc!(iterator);
                getpath!(iterator);
                get_deps!(iterator);
                get_crates!(iterator);
                mkinclude!(iterator);
                 
            }}
mkmod!{marker, { 
                getname!(marker);
                getsrc!(marker);
                getpath!(marker);
                get_deps!(marker);
                get_crates!(marker);
                mkinclude!(marker);
                 
            }}
mkmod!{unchecked_iterator, { 
                getname!(unchecked_iterator);
                getsrc!(unchecked_iterator);
                getpath!(unchecked_iterator);
                get_deps!(unchecked_iterator);
                get_crates!(unchecked_iterator);
                mkinclude!(unchecked_iterator);
                 
            }}
mkuse!{# [unstable (issue = "none" , feature = "inplace_iteration")] pub use self :: marker :: InPlaceIterable ;}
mkuse!{# [unstable (issue = "none" , feature = "trusted_fused")] pub use self :: marker :: TrustedFused ;}
mkuse!{# [unstable (feature = "trusted_step" , issue = "85731")] pub use self :: marker :: TrustedStep ;}
mkuse!{pub (crate) use self :: unchecked_iterator :: UncheckedIterator ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use self :: { accum :: { Product , Sum } , collect :: { Extend , FromIterator , IntoIterator } , double_ended :: DoubleEndedIterator , exact_size :: ExactSizeIterator , iterator :: Iterator , marker :: { FusedIterator , TrustedLen } , } ;}