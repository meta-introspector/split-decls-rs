macro_rules! deps {
    () => {
        Rewrite!();
        Change!();
    };
}

macro_rules! impl_207 {
    () => {
        deps!();
        impl < 'a > From < Change < 'a , '_ , '_ > > for gix_diff :: tree_with_rewrites :: ChangeRef < 'a > { fn from (value : Change < 'a , '_ , '_ >) -> Self { use gix_diff :: tree_with_rewrites :: ChangeRef ; match value { Change :: Addition { location , entry_mode , relation , id , } => ChangeRef :: Addition { location , entry_mode , relation , id : id . detach () , } , Change :: Deletion { location , entry_mode , relation , id , } => ChangeRef :: Deletion { location , entry_mode , relation , id : id . detach () , } , Change :: Modification { location , previous_entry_mode , previous_id , entry_mode , id , } => ChangeRef :: Modification { location , previous_entry_mode , previous_id : previous_id . detach () , entry_mode , id : id . detach () , } , Change :: Rewrite { source_location , source_relation , source_entry_mode , source_id , diff , entry_mode , location , id , relation , copy , } => ChangeRef :: Rewrite { source_location , source_entry_mode , source_relation , source_id : source_id . detach () , diff , entry_mode , id : id . detach () , location , relation , copy , } , } } }
    };
}

impl_207!();