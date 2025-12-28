macro_rules! RenameConflictsVisitor {
    () => {
        struct RenameConflictsVisitor < 'a > { db : & 'a dyn HirDatabase , owner : DefWithBodyId , resolver : Resolver < 'a > , body : & 'a Body , to_be_renamed : BindingId , new_name : Symbol , old_name : Symbol , conflicts : FxHashSet < BindingId > , }
    };
}

RenameConflictsVisitor!()