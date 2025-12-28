macro_rules! eq_visibility {
    () => {
        fn eq_visibility (vis0 : Option < ast :: Visibility > , vis1 : Option < ast :: Visibility >) -> bool { match (vis0 , vis1) { (None , None) => true , (Some (vis0) , Some (vis1)) => vis_eq (& vis0 , & vis1) , _ => false , } }
    };
}

eq_visibility!();