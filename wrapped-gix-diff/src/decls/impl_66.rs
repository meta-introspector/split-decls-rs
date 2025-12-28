macro_rules! deps {
    () => {
        Change!();
        Relation!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl Change { # [doc = " Return the relation this instance may have to other changes."] pub fn relation (& self) -> Option < tree :: visit :: Relation > { match self { Change :: Addition { relation , .. } | Change :: Deletion { relation , .. } | Change :: Rewrite { relation , .. } => * relation , Change :: Modification { .. } => None , } } # [doc = " Return the current mode of this instance."] pub fn entry_mode (& self) -> gix_object :: tree :: EntryMode { match self { Change :: Addition { entry_mode , .. } | Change :: Deletion { entry_mode , .. } | Change :: Modification { entry_mode , .. } | Change :: Rewrite { entry_mode , .. } => * entry_mode , } } # [doc = " Return the current mode of this instance, along with its object id."] pub fn entry_mode_and_id (& self) -> (gix_object :: tree :: EntryMode , & gix_hash :: oid) { match self { Change :: Addition { entry_mode , id , .. } | Change :: Deletion { entry_mode , id , .. } | Change :: Modification { entry_mode , id , .. } | Change :: Rewrite { entry_mode , id , .. } => (* entry_mode , id) , } } # [doc = " Return the *previous* mode and id of the resource where possible, i.e. the source of a rename or copy, or a modification."] pub fn source_entry_mode_and_id (& self) -> (gix_object :: tree :: EntryMode , & gix_hash :: oid) { match self { Change :: Addition { entry_mode , id , .. } | Change :: Deletion { entry_mode , id , .. } | Change :: Modification { previous_entry_mode : entry_mode , previous_id : id , .. } | Change :: Rewrite { source_entry_mode : entry_mode , source_id : id , .. } => (* entry_mode , id) , } } # [doc = " Return the *current* location of the resource, i.e. the destination of a rename or copy, or the"] # [doc = " location at which an addition, deletion or modification took place."] pub fn location (& self) -> & BStr { match self { Change :: Addition { location , .. } | Change :: Deletion { location , .. } | Change :: Modification { location , .. } | Change :: Rewrite { location , .. } => location . as_bstr () , } } # [doc = " Return the *previous* location of the resource where possible, i.e. the source of a rename or copy, or the"] # [doc = " location at which an addition, deletion or modification took place."] pub fn source_location (& self) -> & BStr { match self { Change :: Addition { location , .. } | Change :: Deletion { location , .. } | Change :: Modification { location , .. } => location . as_bstr () , Change :: Rewrite { source_location , .. } => source_location . as_bstr () , } } }
    };
}

impl_66!();