macro_rules! deps {
    () => {
        Change!();
        ChangeRef!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        # [doc = " Lifecycle"] impl Change { # [doc = " Return an attached version of this instance that uses `old_repo` for previous values and `new_repo` for current values."] pub fn to_ref (& self) -> ChangeRef < '_ > { match self { Change :: Addition { location , relation , entry_mode , id , } => ChangeRef :: Addition { location : location . as_bstr () , entry_mode : * entry_mode , id : * id , relation : * relation , } , Change :: Deletion { location , relation , entry_mode , id , } => ChangeRef :: Deletion { location : location . as_bstr () , entry_mode : * entry_mode , id : * id , relation : * relation , } , Change :: Modification { location , previous_entry_mode , previous_id , entry_mode , id , } => ChangeRef :: Modification { location : location . as_bstr () , previous_entry_mode : * previous_entry_mode , previous_id : * previous_id , entry_mode : * entry_mode , id : * id , } , Change :: Rewrite { source_location , source_relation , source_entry_mode , source_id , diff , entry_mode , id , location , relation , copy , } => ChangeRef :: Rewrite { source_location : source_location . as_ref () , source_relation : * source_relation , source_entry_mode : * source_entry_mode , source_id : * source_id , diff : * diff , entry_mode : * entry_mode , id : * id , location : location . as_bstr () , relation : * relation , copy : * copy , } , } } }
    };
}

impl_63!();