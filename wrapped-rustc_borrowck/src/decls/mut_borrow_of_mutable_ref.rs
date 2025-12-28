macro_rules! mut_borrow_of_mutable_ref {
    () => {
        fn mut_borrow_of_mutable_ref (local_decl : & LocalDecl < '_ > , local_name : Option < Symbol >) -> bool { debug ! ("local_info: {:?}, ty.kind(): {:?}" , local_decl . local_info , local_decl . ty . kind ()) ; match * local_decl . local_info () { LocalInfo :: User (mir :: BindingForm :: Var (mir :: VarBindingForm { binding_mode : BindingMode (ByRef :: No , Mutability :: Not) , .. })) => matches ! (local_decl . ty . kind () , ty :: Ref (_ , _ , hir :: Mutability :: Mut)) , LocalInfo :: User (mir :: BindingForm :: ImplicitSelf (kind)) => { kind == hir :: ImplicitSelfKind :: RefMut } _ if Some (kw :: SelfLower) == local_name => { matches ! (local_decl . ty . kind () , ty :: Ref (_ , _ , hir :: Mutability :: Mut)) } _ => false , } }
    };
}

mut_borrow_of_mutable_ref!();