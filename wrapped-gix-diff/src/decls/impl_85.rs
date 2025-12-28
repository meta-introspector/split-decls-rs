macro_rules! deps {
    () => {
        ChangeRef!();
        Relation!();
        ChangeKind!();
        Change!();
    };
}

macro_rules! impl_85 {
    () => {
        deps!();
        impl rewrites :: tracker :: Change for ChangeRef < '_ , '_ > { fn id (& self) -> & gix_hash :: oid { match self { ChangeRef :: Addition { id , .. } | ChangeRef :: Deletion { id , .. } | ChangeRef :: Modification { id , .. } => { id . as_ref () } ChangeRef :: Rewrite { .. } => { unreachable ! ("BUG") } } } fn relation (& self) -> Option < Relation > { None } fn kind (& self) -> ChangeKind { match self { ChangeRef :: Addition { .. } => ChangeKind :: Addition , ChangeRef :: Deletion { .. } => ChangeKind :: Deletion , ChangeRef :: Modification { .. } => ChangeKind :: Modification , ChangeRef :: Rewrite { .. } => { unreachable ! ("BUG: rewrites can't be determined ahead of time") } } } fn entry_mode (& self) -> tree :: EntryMode { match self { ChangeRef :: Addition { entry_mode , .. } | ChangeRef :: Deletion { entry_mode , .. } | ChangeRef :: Modification { entry_mode , .. } | ChangeRef :: Rewrite { entry_mode , .. } => { entry_mode . to_tree_entry_mode () . unwrap_or (tree :: EntryKind :: Tree . into ()) } } } fn id_and_entry_mode (& self) -> (& gix_hash :: oid , tree :: EntryMode) { match self { ChangeRef :: Addition { id , entry_mode , .. } | ChangeRef :: Deletion { id , entry_mode , .. } | ChangeRef :: Modification { id , entry_mode , .. } | ChangeRef :: Rewrite { id , entry_mode , .. } => { (id , entry_mode . to_tree_entry_mode () . unwrap_or (tree :: EntryKind :: Tree . into ()) ,) } } } }
    };
}

impl_85!();