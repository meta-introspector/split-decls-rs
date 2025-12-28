macro_rules! work_products_path {
    () => {
        pub (crate) fn work_products_path (sess : & Session) -> PathBuf { in_incr_comp_dir_sess (sess , WORK_PRODUCTS_FILENAME) }
    };
}

work_products_path!();