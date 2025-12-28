macro_rules! tests {
    () => {
        # [cfg (not (target_vendor = "uwp"))] # [cfg (test)] mod tests { use super :: home_dir_inner ; use std :: env ; use std :: ops :: Deref ; use std :: path :: { Path , PathBuf } ; # [test] fn test_with_without () { let olduserprofile = env :: var_os ("USERPROFILE") . unwrap () ; unsafe { env :: remove_var ("HOME") ; env :: remove_var ("USERPROFILE") ; } assert_eq ! (home_dir_inner () , Some (PathBuf :: from (olduserprofile))) ; let home = Path :: new (r"C:\Users\foo tar baz") ; unsafe { env :: set_var ("HOME" , home . as_os_str ()) ; } assert_ne ! (home_dir_inner () . as_ref () . map (Deref :: deref) , Some (home)) ; unsafe { env :: set_var ("USERPROFILE" , home . as_os_str ()) ; } assert_eq ! (home_dir_inner () . as_ref () . map (Deref :: deref) , Some (home)) ; } }
    };
}

tests!()