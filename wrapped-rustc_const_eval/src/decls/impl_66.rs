macro_rules! deps {
    () => {
        State!();
        Qualif!();
        ConstCx!();
        TransferFunction!();
        Place!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl < 'mir , 'tcx , Q > TransferFunction < 'mir , 'tcx , Q > where Q : Qualif , { fn new (ccx : & 'mir ConstCx < 'mir , 'tcx > , state : & 'mir mut State) -> Self { TransferFunction { ccx , state , _qualif : PhantomData } } fn initialize_state (& mut self) { self . state . qualif . clear () ; self . state . borrow . clear () ; for arg in self . ccx . body . args_iter () { let arg_ty = self . ccx . body . local_decls [arg] . ty ; if Q :: in_any_value_of_ty (self . ccx , arg_ty) { self . state . qualif . insert (arg) ; } } } fn assign_qualif_direct (& mut self , place : & mir :: Place < 'tcx > , mut value : bool) { debug_assert ! (! place . is_indirect ()) ; if ! value { for (base , _elem) in place . iter_projections () { let base_ty = base . ty (self . ccx . body , self . ccx . tcx) ; if base_ty . ty . is_union () && Q :: in_any_value_of_ty (self . ccx , base_ty . ty) { value = true ; break ; } } } match (value , place . as_ref ()) { (true , mir :: PlaceRef { local , .. }) => { self . state . qualif . insert (local) ; } (false , mir :: PlaceRef { local : _ , projection : & [] }) => { } _ => { } } } fn apply_call_return_effect (& mut self , _block : BasicBlock , return_places : CallReturnPlaces < '_ , 'tcx > ,) { return_places . for_each (| place | { let return_ty = place . ty (self . ccx . body , self . ccx . tcx) . ty ; let qualif = Q :: in_any_value_of_ty (self . ccx , return_ty) ; if ! place . is_indirect () { self . assign_qualif_direct (& place , qualif) ; } }) ; } fn address_of_allows_mutation (& self) -> bool { true } fn ref_allows_mutation (& self , kind : mir :: BorrowKind , place : mir :: Place < 'tcx >) -> bool { match kind { mir :: BorrowKind :: Mut { .. } => true , mir :: BorrowKind :: Shared | mir :: BorrowKind :: Fake (_) => { self . shared_borrow_allows_mutation (place) } } } # [doc = " `&` only allow mutation if the borrowed place is `!Freeze`."] # [doc = ""] # [doc = " This assumes that it is UB to take the address of a struct field whose type is"] # [doc = " `Freeze`, then use pointer arithmetic to derive a pointer to a *different* field of"] # [doc = " that same struct whose type is `!Freeze`. If we decide that this is not UB, we will"] # [doc = " have to check the type of the borrowed **local** instead of the borrowed **place**"] # [doc = " below. See [rust-lang/unsafe-code-guidelines#134]."] # [doc = ""] # [doc = " [rust-lang/unsafe-code-guidelines#134]: https://github.com/rust-lang/unsafe-code-guidelines/issues/134"] fn shared_borrow_allows_mutation (& self , place : mir :: Place < 'tcx >) -> bool { ! place . ty (self . ccx . body , self . ccx . tcx) . ty . is_freeze (self . ccx . tcx , self . ccx . typing_env) } }
    };
}

impl_66!()