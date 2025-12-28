macro_rules! check_map_indexed {
    () => {
        # [test] fn check_map_indexed () { let a = [1 , 2 , 3] ; is_indexed (a . par_iter () . map (| x | x)) ; }
    };
}

check_map_indexed!()