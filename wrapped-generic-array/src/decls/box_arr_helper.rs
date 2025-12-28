macro_rules! box_arr_helper {
    () => {
        # [cfg (feature = "alloc")] # [doc (hidden)] # [macro_export] macro_rules ! box_arr_helper { (@ unit $ e : expr) => { () } ; }
    };
}

box_arr_helper!();