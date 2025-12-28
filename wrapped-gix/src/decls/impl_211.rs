macro_rules! deps {
    () => {
        Rewrite!();
        Change!();
        Id!();
    };
}

macro_rules! impl_211 {
    () => {
        deps!();
        impl Change < '_ , '_ , '_ > { # [doc = " Return the current ID of the change."] pub fn id (& self) -> crate :: Id < '_ > { match self { Change :: Addition { id , .. } | Change :: Deletion { id , .. } | Change :: Modification { id , .. } | Change :: Rewrite { id , .. } => * id , } } # [doc = " Return the location of this instance."] pub fn location (& self) -> & BStr { match self { Change :: Addition { location , .. } | Change :: Deletion { location , .. } | Change :: Modification { location , .. } | Change :: Rewrite { location , .. } => location . as_bstr () , } } # [doc = " Return the current mode of this instance."] pub fn entry_mode (& self) -> gix_object :: tree :: EntryMode { match self { Change :: Addition { entry_mode , .. } | Change :: Deletion { entry_mode , .. } | Change :: Modification { entry_mode , .. } | Change :: Rewrite { entry_mode , .. } => * entry_mode , } } }
    };
}

impl_211!()