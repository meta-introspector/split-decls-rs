macro_rules! deps {
    () => {
        Rewrite!();
        Repository!();
        Note!();
        Change!();
    };
}

macro_rules! impl_208 {
    () => {
        deps!();
        impl < 'a , 'old , 'new > Change < 'a , 'old , 'new > { # [doc = " Convert `change` into this instance type, attaching the `old_repo` and `new_repo` to each side respectively."] # [doc = " Note that both repos are typically the same."] pub fn from_change_ref (change : gix_diff :: tree_with_rewrites :: ChangeRef < 'a > , old_repo : & 'old Repository , new_repo : & 'new Repository ,) -> Self { use gix_diff :: tree_with_rewrites :: ChangeRef ; match change { ChangeRef :: Addition { location , entry_mode , relation , id , } => Change :: Addition { location , entry_mode , relation , id : id . attach (new_repo) , } , ChangeRef :: Deletion { location , entry_mode , relation , id , } => Change :: Deletion { location , entry_mode , relation , id : id . attach (old_repo) , } , ChangeRef :: Modification { location , previous_entry_mode , previous_id , entry_mode , id , } => Change :: Modification { location , previous_entry_mode , entry_mode , previous_id : previous_id . attach (old_repo) , id : id . attach (new_repo) , } , ChangeRef :: Rewrite { source_location , source_entry_mode , source_relation , source_id , diff , entry_mode , id , location , relation , copy , } => Change :: Rewrite { source_location , source_relation , source_entry_mode , source_id : source_id . attach (old_repo) , diff , entry_mode , location , id : id . attach (new_repo) , relation , copy , } , } } }
    };
}

impl_208!()