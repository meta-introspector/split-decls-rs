macro_rules! __detect_target_features {
    () => {
        # [macro_export] # [doc (hidden)] macro_rules ! __detect_target_features { ($ ($ tf : tt) ,+) => { false } ; }
    };
}

__detect_target_features!()