macro_rules! deps {
    () => {
        Level!();
    };
}

macro_rules! coarse {
    () => {
        deps!();
        # [doc = " Create a new [coarse][Level::Coarse] span."] # [macro_export] macro_rules ! coarse { (target : $ target : expr , $ name : expr , $ ($ field : tt) *) => { $ crate :: span ! (target : $ target , $ crate :: Level :: Coarse , $ name , $ ($ field) *) } ; (target : $ target : expr , $ name : expr) => { $ crate :: coarse ! (target : $ target , $ name ,) } ; ($ name : expr , $ ($ field : tt) *) => { $ crate :: span ! (target : module_path ! () , $ crate :: Level :: Coarse , $ name , $ ($ field) *) } ; ($ name : expr) => { $ crate :: coarse ! ($ name ,) } ; }
    };
}

coarse!()