macro_rules! deps {
    () => {
        Change!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl Change { # [doc = " Return the current object id."] pub fn oid (& self) -> & gix_hash :: oid { match self { Change :: Addition { oid , .. } | Change :: Deletion { oid , .. } | Change :: Modification { oid , .. } => oid , } } # [doc = " Return the current tree entry mode."] pub fn entry_mode (& self) -> EntryMode { match self { Change :: Addition { entry_mode , .. } | Change :: Deletion { entry_mode , .. } | Change :: Modification { entry_mode , .. } => * entry_mode , } } # [doc = " Return the current object id and tree entry mode of a change."] pub fn oid_and_entry_mode (& self) -> (& gix_hash :: oid , EntryMode) { match self { Change :: Addition { oid , entry_mode , relation : _ , } | Change :: Deletion { oid , entry_mode , relation : _ , } | Change :: Modification { oid , entry_mode , .. } => (oid , * entry_mode) , } } }
    };
}

impl_44!();