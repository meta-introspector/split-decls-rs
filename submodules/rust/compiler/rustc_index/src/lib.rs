mkmod!{bit_set, { 
                getname!(bit_set);
                getsrc!(bit_set);
                getpath!(bit_set);
                get_deps!(bit_set);
                get_crates!(bit_set);
                mkinclude!(bit_set);
                 
            }}
mkmod!{interval, { 
                getname!(interval);
                getsrc!(interval);
                getpath!(interval);
                get_deps!(interval);
                get_crates!(interval);
                mkinclude!(interval);
                 
            }}
mkmod!{idx, { 
                getname!(idx);
                getsrc!(idx);
                getpath!(idx);
                get_deps!(idx);
                get_crates!(idx);
                mkinclude!(idx);
                 
            }}
mkmod!{slice, { 
                getname!(slice);
                getsrc!(slice);
                getpath!(slice);
                get_deps!(slice);
                get_crates!(slice);
                mkinclude!(slice);
                 
            }}
mkmod!{vec, { 
                getname!(vec);
                getsrc!(vec);
                getpath!(vec);
                get_deps!(vec);
                get_crates!(vec);
                mkinclude!(vec);
                 
            }}
mkuse!{pub use idx :: { Idx , IntoSliceIdx } ;}
mkuse!{pub use rustc_index_macros :: newtype_index ;}
mkuse!{pub use slice :: IndexSlice ;}
mkuse!{# [doc (no_inline)] pub use vec :: IndexVec ;}
mkitem!{# [doc = " Type size assertion. The first argument is a type and the second argument is its expected size."] # [doc = ""] # [doc = " <div class=\"warning\">"] # [doc = ""] # [doc = " Emitting hard errors from size assertions like this is generally not"] # [doc = " recommended, especially in libraries, because they can cause build failures if the layout"] # [doc = " algorithm or dependencies change. Here in rustc we control the toolchain and layout algorithm,"] # [doc = " so the former is not a problem. For the latter we have a lockfile as rustc is an application and"] # [doc = " precompiled library."] # [doc = ""] # [doc = " Short version: Don't copy this macro into your own code. Use a `#[test]` instead."] # [doc = ""] # [doc = " </div>"] # [macro_export] # [cfg (not (feature = "rustc_randomized_layouts"))] macro_rules ! static_assert_size { ($ ty : ty , $ size : expr) => { const _ : [() ; $ size] = [() ; :: std :: mem :: size_of ::<$ ty > ()] ; } ; }}
mkitem!{# [macro_export] # [cfg (feature = "rustc_randomized_layouts")] macro_rules ! static_assert_size { ($ ty : ty , $ size : expr) => { const _ : (usize , usize) = ($ size , :: std :: mem :: size_of ::<$ ty > ()) ; } ; }}