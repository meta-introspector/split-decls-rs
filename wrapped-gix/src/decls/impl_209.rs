macro_rules! deps {
    () => {
        Change!();
        Rewrite!();
    };
}

macro_rules! impl_209 {
    () => {
        deps!();
        # [doc = " Lifecycle"] impl Change < '_ , '_ , '_ > { # [doc = " Detach the repository instance to obtain a fully-owned version"] pub fn detach (self) -> ChangeDetached { match self { Change :: Addition { entry_mode , id , location , relation , } => ChangeDetached :: Addition { entry_mode , id : id . detach () , location : location . to_owned () , relation , } , Change :: Deletion { entry_mode , id , location , relation , } => ChangeDetached :: Deletion { entry_mode , id : id . detach () , location : location . to_owned () , relation , } , Change :: Modification { previous_entry_mode , previous_id , entry_mode , id , location , } => ChangeDetached :: Modification { previous_entry_mode , previous_id : previous_id . detach () , entry_mode , id : id . detach () , location : location . to_owned () , } , Change :: Rewrite { source_location , source_relation , source_entry_mode , source_id , diff , entry_mode , id , relation , copy , location , } => ChangeDetached :: Rewrite { source_location : source_location . to_owned () , source_entry_mode , source_relation , source_id : source_id . detach () , diff , entry_mode , id : id . detach () , copy , location : location . to_owned () , relation , } , } } }
    };
}

impl_209!()