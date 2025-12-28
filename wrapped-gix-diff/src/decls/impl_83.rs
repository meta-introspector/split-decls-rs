macro_rules! deps {
    () => {
        ChangeRef!();
        Change!();
    };
}

macro_rules! impl_83 {
    () => {
        deps!();
        impl ChangeRef < '_ , '_ > { # [doc = " Copy everything into an owned version of this instance."] pub fn into_owned (self) -> Change { match self { ChangeRef :: Addition { location , index , entry_mode , id , } => ChangeRef :: Addition { location : Cow :: Owned (location . into_owned ()) , index , entry_mode , id : Cow :: Owned (id . into_owned ()) , } , ChangeRef :: Deletion { location , index , entry_mode , id , } => ChangeRef :: Deletion { location : Cow :: Owned (location . into_owned ()) , index , entry_mode , id : Cow :: Owned (id . into_owned ()) , } , ChangeRef :: Modification { location , previous_index , previous_entry_mode , previous_id , index , entry_mode , id , } => ChangeRef :: Modification { location : Cow :: Owned (location . into_owned ()) , previous_index , previous_entry_mode , previous_id : Cow :: Owned (previous_id . into_owned ()) , index , entry_mode , id : Cow :: Owned (id . into_owned ()) , } , ChangeRef :: Rewrite { source_location , source_index , source_entry_mode , source_id , location , index , entry_mode , id , copy , } => ChangeRef :: Rewrite { source_location : Cow :: Owned (source_location . into_owned ()) , source_index , source_entry_mode , source_id : Cow :: Owned (source_id . into_owned ()) , location : Cow :: Owned (location . into_owned ()) , index , entry_mode , id : Cow :: Owned (id . into_owned ()) , copy , } , } } }
    };
}

impl_83!()