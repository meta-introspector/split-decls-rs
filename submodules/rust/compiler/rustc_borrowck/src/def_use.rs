mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: mir :: visit :: { MutatingUseContext , NonMutatingUseContext , NonUseContext , PlaceContext , } ;}
mkitem!{mkenum!{# [derive (Eq , PartialEq , Clone)] pub (crate) enum DefUse { Def , Use , Drop , }}}

macro_rules! categorize_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function categorize in module {}", module_path!());
    };
}

mkfn!{
    categorize_introspect!();
    pub (crate) fn categorize (context : PlaceContext) -> Option < DefUse > { match context { PlaceContext :: MutatingUse (MutatingUseContext :: Store) | PlaceContext :: MutatingUse (MutatingUseContext :: Call) | PlaceContext :: MutatingUse (MutatingUseContext :: AsmOutput) | PlaceContext :: MutatingUse (MutatingUseContext :: Yield) | PlaceContext :: NonUse (NonUseContext :: StorageLive) | PlaceContext :: NonUse (NonUseContext :: StorageDead) => Some (DefUse :: Def) , PlaceContext :: NonMutatingUse (NonMutatingUseContext :: Projection) | PlaceContext :: MutatingUse (MutatingUseContext :: Projection) | PlaceContext :: MutatingUse (MutatingUseContext :: Borrow) | PlaceContext :: NonMutatingUse (NonMutatingUseContext :: SharedBorrow) | PlaceContext :: NonMutatingUse (NonMutatingUseContext :: FakeBorrow) | PlaceContext :: NonMutatingUse (NonMutatingUseContext :: PlaceMention) | PlaceContext :: NonUse (NonUseContext :: AscribeUserTy (_)) | PlaceContext :: MutatingUse (MutatingUseContext :: RawBorrow) | PlaceContext :: NonMutatingUse (NonMutatingUseContext :: RawBorrow) | PlaceContext :: NonMutatingUse (NonMutatingUseContext :: Inspect) | PlaceContext :: NonMutatingUse (NonMutatingUseContext :: Copy) | PlaceContext :: NonMutatingUse (NonMutatingUseContext :: Move) | PlaceContext :: MutatingUse (MutatingUseContext :: Retag) => Some (DefUse :: Use) , PlaceContext :: MutatingUse (MutatingUseContext :: Drop) => Some (DefUse :: Drop) , PlaceContext :: NonUse (NonUseContext :: VarDebugInfo) => None , PlaceContext :: NonUse (NonUseContext :: BackwardIncompatibleDropHint) => None , PlaceContext :: MutatingUse (MutatingUseContext :: Deinit | MutatingUseContext :: SetDiscriminant) => { bug ! ("These statements are not allowed in this MIR phase") } } }
}