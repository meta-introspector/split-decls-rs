macro_rules! deps {
    () => {
        DeleteWorkProduct!();
    };
}

macro_rules! delete_workproduct_files {
    () => {
        deps!();
        # [doc = " Removes files for a given work product."] pub (crate) fn delete_workproduct_files (sess : & Session , work_product : & WorkProduct) { for (_ , path) in work_product . saved_files . items () . into_sorted_stable_ord () { let path = in_incr_comp_dir_sess (sess , path) ; if let Err (err) = std_fs :: remove_file (& path) { sess . dcx () . emit_warn (errors :: DeleteWorkProduct { path : & path , err }) ; } } }
    };
}

delete_workproduct_files!();