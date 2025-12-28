macro_rules! deps {
    () => {
        Change!();
        Rewrite!();
        Repository!();
        TreeDiffChangeExt!();
    };
}

macro_rules! impl_210 {
    () => {
        deps!();
        impl crate :: ext :: TreeDiffChangeExt for gix_diff :: tree_with_rewrites :: Change { fn attach < 'old , 'new > (& self , old_repo : & 'old Repository , new_repo : & 'new Repository) -> Change < '_ , 'old , 'new > { match self { ChangeDetached :: Addition { entry_mode , id , location , relation , } => Change :: Addition { entry_mode : * entry_mode , id : id . attach (new_repo) , location : location . as_bstr () , relation : * relation , } , ChangeDetached :: Deletion { entry_mode , id , location , relation , } => Change :: Deletion { entry_mode : * entry_mode , id : id . attach (old_repo) , location : location . as_bstr () , relation : * relation , } , ChangeDetached :: Modification { previous_entry_mode , previous_id , entry_mode , id , location , } => Change :: Modification { previous_entry_mode : * previous_entry_mode , previous_id : previous_id . attach (old_repo) , entry_mode : * entry_mode , id : id . attach (new_repo) , location : location . as_bstr () , } , ChangeDetached :: Rewrite { source_location , source_relation , source_entry_mode , source_id , diff , entry_mode , id , copy , location , relation , } => Change :: Rewrite { source_location : source_location . as_ref () , source_relation : * source_relation , source_entry_mode : * source_entry_mode , source_id : source_id . attach (old_repo) , diff : * diff , entry_mode : * entry_mode , id : id . attach (new_repo) , copy : * copy , relation : * relation , location : location . as_bstr () , } , } } }
    };
}

impl_210!()