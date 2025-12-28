macro_rules! deps {
    () => {
        Mode!();
        ChangeRef!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl ChangeRef < '_ , '_ > { # [doc = " Return all shared fields among all variants: `(location, index, entry_mode, id)`"] # [doc = ""] # [doc = " In case of rewrites, the fields return to the current change."] # [doc = ""] # [doc = " Note that there are also more specific accessors in case you only need to access to one of"] # [doc = " these fields individually."] # [doc = ""] # [doc = " See [`ChangeRef::location()`], [`ChangeRef::index()`], [`ChangeRef::entry_mode()`] and"] # [doc = " [`ChangeRef::id()`]."] pub fn fields (& self) -> (& BStr , usize , gix_index :: entry :: Mode , & gix_hash :: oid) { match self { ChangeRef :: Addition { location , index , entry_mode , id , .. } | ChangeRef :: Deletion { location , index , entry_mode , id , .. } | ChangeRef :: Modification { location , index , entry_mode , id , .. } | ChangeRef :: Rewrite { location , index , entry_mode , id , .. } => (location . as_ref () , * index , * entry_mode , id) , } } # [doc = " Return the `location`, in the case of rewrites referring to the current change."] pub fn location (& self) -> & BStr { match self { ChangeRef :: Addition { location , .. } | ChangeRef :: Deletion { location , .. } | ChangeRef :: Modification { location , .. } | ChangeRef :: Rewrite { location , .. } => location . as_ref () , } } # [doc = " Return the `index`, in the case of rewrites referring to the current change."] pub fn index (& self) -> usize { match self { ChangeRef :: Addition { index , .. } | ChangeRef :: Deletion { index , .. } | ChangeRef :: Modification { index , .. } | ChangeRef :: Rewrite { index , .. } => * index , } } # [doc = " Return the `entry_mode`, in the case of rewrites referring to the current change."] pub fn entry_mode (& self) -> gix_index :: entry :: Mode { match self { ChangeRef :: Addition { entry_mode , .. } | ChangeRef :: Deletion { entry_mode , .. } | ChangeRef :: Modification { entry_mode , .. } | ChangeRef :: Rewrite { entry_mode , .. } => * entry_mode , } } # [doc = " Return the `id`, in the case of rewrites referring to the current change."] pub fn id (& self) -> & gix_hash :: oid { match self { ChangeRef :: Addition { id , .. } | ChangeRef :: Deletion { id , .. } | ChangeRef :: Modification { id , .. } | ChangeRef :: Rewrite { id , .. } => id , } } }
    };
}

impl_84!()