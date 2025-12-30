// Generated macro for impl_626 (impl)
macro_rules! Depcrate_submoduleimpl_626 {
() => {
// Module: crate::submodule
// Provides: {"impl_626"}
// Dependencies: {}
impl < 'repo > SharedState < 'repo > { pub (crate) fn new (repo : & 'repo Repository , modules : ModulesSnapshot) -> Self { SharedState { repo , modules , is_active : RefCell :: new (None) , index : RefCell :: new (None) , } } fn index (& self) -> Result < Ref < '_ , IndexPersistedOrInMemory > , crate :: repository :: index_or_load_from_head :: Error > { { let mut state = self . index . borrow_mut () ; if state . is_none () { * state = self . repo . index_or_load_from_head () ? . into () ; } } Ok (Ref :: map (self . index . borrow () , | opt | { opt . as_ref () . expect ("just initialized") })) } fn active_state_mut (& self ,) -> Result < (RefMut < '_ , IsActivePlatform > , RefMut < '_ , gix_worktree :: Stack >) , is_active :: Error > { let mut state = self . is_active . borrow_mut () ; if state . is_none () { let platform = self . modules . is_active_platform (& self . repo . config . resolved , self . repo . config . pathspec_defaults () ?) ? ; let index = self . index () ? ; let attributes = self . repo . attributes_only (& index , gix_worktree :: stack :: state :: attributes :: Source :: WorktreeThenIdMapping . adjust_for_bare (self . repo . is_bare ()) ,) ? . detach () ; * state = Some (IsActiveState { platform , attributes }) ; } Ok (RefMut :: map_split (state , | opt | { let state = opt . as_mut () . expect ("populated above") ; (& mut state . platform , & mut state . attributes) })) } }
};
}
