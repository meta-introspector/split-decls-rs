macro_rules! deps {
    () => {
        Level!();
    };
}

macro_rules! detail {
    () => {
        deps!();
        # [doc = " Create a new [detail][Level::Detail] span."] # [macro_export] macro_rules ! detail { (target : $ target : expr , $ name : expr , $ ($ field : tt) *) => { $ crate :: span ! (target : $ target , $ crate :: Level :: Detail , $ name , $ ($ field) *) } ; (target : $ target : expr , $ name : expr) => { $ crate :: detail ! (target : $ target , $ name ,) } ; ($ name : expr , $ ($ field : tt) *) => { $ crate :: span ! (target : module_path ! () , $ crate :: Level :: Detail , $ name , $ ($ field) *) } ; ($ name : expr) => { $ crate :: coarse ! ($ name ,) } ; }
    };
}

detail!()