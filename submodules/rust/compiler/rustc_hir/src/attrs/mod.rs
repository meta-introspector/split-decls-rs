mkuse!{pub use data_structures :: * ;}
mkuse!{pub use encode_cross_crate :: EncodeCrossCrate ;}
mkuse!{pub use pretty_printing :: PrintAttribute ;}
mkmod!{data_structures, { 
                getname!(data_structures);
                getsrc!(data_structures);
                getpath!(data_structures);
                get_deps!(data_structures);
                get_crates!(data_structures);
                mkinclude!(data_structures);
                 
            }}
mkmod!{encode_cross_crate, { 
                getname!(encode_cross_crate);
                getsrc!(encode_cross_crate);
                getpath!(encode_cross_crate);
                get_deps!(encode_cross_crate);
                get_crates!(encode_cross_crate);
                mkinclude!(encode_cross_crate);
                 
            }}
mkmod!{pretty_printing, { 
                getname!(pretty_printing);
                getsrc!(pretty_printing);
                getpath!(pretty_printing);
                get_deps!(pretty_printing);
                get_crates!(pretty_printing);
                mkinclude!(pretty_printing);
                 
            }}
mkitem!{# [doc = " Finds attributes in sequences of attributes by pattern matching."] # [doc = ""] # [doc = " A little like `matches` but for attributes."] # [doc = ""] # [doc = " ```rust,ignore (illustrative)"] # [doc = " // finds the repr attribute"] # [doc = " if let Some(r) = find_attr!(attrs, AttributeKind::Repr(r) => r) {"] # [doc = ""] # [doc = " }"] # [doc = ""] # [doc = " // checks if one has matched"] # [doc = " if find_attr!(attrs, AttributeKind::Repr(_)) {"] # [doc = ""] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Often this requires you to first end up with a list of attributes."] # [doc = " A common way to get those is through `tcx.get_all_attrs(did)`"] # [macro_export] macro_rules ! find_attr { ($ attributes_list : expr , $ pattern : pat $ (if $ guard : expr) ?) => { { $ crate :: find_attr ! ($ attributes_list , $ pattern $ (if $ guard) ? => ()) . is_some () } } ; ($ attributes_list : expr , $ pattern : pat $ (if $ guard : expr) ? => $ e : expr) => { { 'done : { for i in $ attributes_list { let i : & rustc_hir :: Attribute = i ; match i { rustc_hir :: Attribute :: Parsed ($ pattern) $ (if $ guard) ? => { break 'done Some ($ e) ; } _ => { } } } None } } } ; }}