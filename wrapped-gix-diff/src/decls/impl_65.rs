macro_rules! deps {
    () => {
        ChangeRef!();
        Relation!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl < 'a > ChangeRef < 'a > { # [doc = " Return the relation this instance may have to other changes."] pub fn relation (& self) -> Option < tree :: visit :: Relation > { match self { ChangeRef :: Addition { relation , .. } | ChangeRef :: Deletion { relation , .. } | ChangeRef :: Rewrite { relation , .. } => * relation , ChangeRef :: Modification { .. } => None , } } # [doc = " Return the current mode of this instance."] pub fn entry_mode (& self) -> gix_object :: tree :: EntryMode { match self { ChangeRef :: Addition { entry_mode , .. } | ChangeRef :: Deletion { entry_mode , .. } | ChangeRef :: Modification { entry_mode , .. } | ChangeRef :: Rewrite { entry_mode , .. } => * entry_mode , } } # [doc = " Return the current mode of this instance, along with its object id."] pub fn entry_mode_and_id (& self) -> (gix_object :: tree :: EntryMode , & gix_hash :: oid) { match self { ChangeRef :: Addition { entry_mode , id , .. } | ChangeRef :: Deletion { entry_mode , id , .. } | ChangeRef :: Modification { entry_mode , id , .. } | ChangeRef :: Rewrite { entry_mode , id , .. } => (* entry_mode , id) , } } # [doc = " Return the *previous* mode and id of the resource where possible, i.e. the source of a rename or copy, or a modification."] pub fn source_entry_mode_and_id (& self) -> (gix_object :: tree :: EntryMode , & gix_hash :: oid) { match self { ChangeRef :: Addition { entry_mode , id , .. } | ChangeRef :: Deletion { entry_mode , id , .. } | ChangeRef :: Modification { previous_entry_mode : entry_mode , previous_id : id , .. } | ChangeRef :: Rewrite { source_entry_mode : entry_mode , source_id : id , .. } => (* entry_mode , id) , } } # [doc = " Return the *current* location of the resource, i.e. the destination of a rename or copy, or the"] # [doc = " location at which an addition, deletion or modification took place."] pub fn location (& self) -> & 'a BStr { match self { ChangeRef :: Addition { location , .. } | ChangeRef :: Deletion { location , .. } | ChangeRef :: Modification { location , .. } | ChangeRef :: Rewrite { location , .. } => location , } } # [doc = " Return the *previous* location of the resource where possible, i.e. the source of a rename or copy, or the"] # [doc = " location at which an addition, deletion or modification took place."] pub fn source_location (& self) -> & BStr { match self { ChangeRef :: Addition { location , .. } | ChangeRef :: Deletion { location , .. } | ChangeRef :: Modification { location , .. } => location , ChangeRef :: Rewrite { source_location , .. } => source_location , } } }
    };
}

impl_65!();