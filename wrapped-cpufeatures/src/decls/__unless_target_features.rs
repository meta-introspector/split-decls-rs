macro_rules! __unless_target_features {
    () => {
        # [macro_export] # [doc (hidden)] macro_rules ! __unless_target_features { ($ ($ tf : tt) ,+ => $ body : expr) => { false } ; }
    };
}

__unless_target_features!()