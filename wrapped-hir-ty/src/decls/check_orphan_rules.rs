macro_rules! deps {
    () => {
        HirDatabase!();
    };
}

macro_rules! check_orphan_rules {
    () => {
        deps!();
        # [doc = " Checks whether the impl satisfies the orphan rules."] # [doc = ""] # [doc = " Given `impl<P1..=Pn> Trait<T1..=Tn> for T0`, an `impl`` is valid only if at least one of the following is true:"] # [doc = " - Trait is a local trait"] # [doc = " - All of"] # [doc = "   - At least one of the types `T0..=Tn`` must be a local type. Let `Ti`` be the first such type."] # [doc = "   - No uncovered type parameters `P1..=Pn` may appear in `T0..Ti`` (excluding `Ti`)"] pub fn check_orphan_rules < 'db > (db : & 'db dyn HirDatabase , impl_ : ImplId) -> bool { let Some (impl_trait) = db . impl_trait (impl_) else { return true ; } ; let local_crate = impl_ . lookup (db) . container . krate () ; let is_local = | tgt_crate | tgt_crate == local_crate ; let trait_ref = impl_trait . instantiate_identity () ; let trait_id = trait_ref . def_id . 0 ; if is_local (trait_id . module (db) . krate ()) { return true ; } let unwrap_fundamental = | mut ty : Ty < 'db > | { loop { match ty . kind () { TyKind :: Ref (_ , referenced , _) => ty = referenced , TyKind :: Adt (adt_def , subs) => { let AdtId :: StructId (s) = adt_def . def_id () . 0 else { break ty ; } ; let struct_signature = db . struct_signature (s) ; if struct_signature . flags . contains (StructFlags :: FUNDAMENTAL) { let next = subs . types () . next () ; match next { Some (it) => ty = it , None => break ty , } } else { break ty ; } } _ => break ty , } } } ; let is_not_orphan = trait_ref . args . types () . any (| ty | match unwrap_fundamental (ty) . kind () { TyKind :: Adt (adt_def , _) => is_local (adt_def . def_id () . 0 . module (db) . krate ()) , TyKind :: Error (_) => true , TyKind :: Dynamic (it , _) => { it . principal_def_id () . is_some_and (| trait_id | is_local (trait_id . 0 . module (db) . krate ())) } _ => false , }) ; # [allow (clippy :: let_and_return)] is_not_orphan }
    };
}

check_orphan_rules!()