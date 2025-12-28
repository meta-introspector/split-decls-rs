macro_rules! deps {
    () => {
        BuiltinSubmoduleStatus!();
        Entry!();
        Item!();
        Collect!();
    };
}

macro_rules! impl_1036 {
    () => {
        deps!();
        impl < 'index > gix_status :: index_as_worktree_with_renames :: VisitEntry < 'index > for Collect { type ContentChange = < gix_status :: index_as_worktree :: traits :: FastEq as gix_status :: index_as_worktree :: traits :: CompareBlobs > :: Output ; type SubmoduleStatus = < BuiltinSubmoduleStatus as gix_status :: index_as_worktree :: traits :: SubmoduleStatus > :: Output ; fn visit_entry (& mut self , entry : gix_status :: index_as_worktree_with_renames :: Entry < 'index , Self :: ContentChange , Self :: SubmoduleStatus > ,) { let item = Item :: IndexWorktree (entry . into ()) ; # [cfg (feature = "parallel")] self . tx . send (item) . ok () ; # [cfg (not (feature = "parallel"))] self . items . push (item) ; } }
    };
}

impl_1036!()