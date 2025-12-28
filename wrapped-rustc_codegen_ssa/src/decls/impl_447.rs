macro_rules! deps {
    () => {
        BuilderMethods!();
        LocalAnalyzer!();
        LocalKind!();
    };
}

macro_rules! impl_447 {
    () => {
        deps!();
        impl < 'a , 'b , 'tcx , Bx : BuilderMethods < 'b , 'tcx > > Visitor < 'tcx > for LocalAnalyzer < 'a , 'b , 'tcx , Bx > { fn visit_assign (& mut self , place : & mir :: Place < 'tcx > , rvalue : & mir :: Rvalue < 'tcx > , location : Location ,) { debug ! ("visit_assign(place={:?}, rvalue={:?})" , place , rvalue) ; if let Some (local) = place . as_local () { self . define (local , DefLocation :: Assignment (location)) ; } else { self . visit_place (place , PlaceContext :: MutatingUse (MutatingUseContext :: Store) , location) ; } self . visit_rvalue (rvalue , location) ; } fn visit_place (& mut self , place : & mir :: Place < 'tcx > , context : PlaceContext , location : Location) { debug ! ("visit_place(place={:?}, context={:?})" , place , context) ; self . process_place (& place . as_ref () , context , location) ; } fn visit_local (& mut self , local : mir :: Local , context : PlaceContext , location : Location) { match context { PlaceContext :: MutatingUse (MutatingUseContext :: Call) => { let call = location . block ; let TerminatorKind :: Call { target , .. } = self . fx . mir . basic_blocks [call] . terminator () . kind else { bug ! () } ; self . define (local , DefLocation :: CallReturn { call , target }) ; } PlaceContext :: NonUse (_) | PlaceContext :: NonMutatingUse (NonMutatingUseContext :: PlaceMention) | PlaceContext :: MutatingUse (MutatingUseContext :: Retag) => { } PlaceContext :: NonMutatingUse (NonMutatingUseContext :: Copy | NonMutatingUseContext :: Move | NonMutatingUseContext :: Inspect ,) => match & mut self . locals [local] { LocalKind :: ZST => { } LocalKind :: Memory => { } LocalKind :: SSA (def) if def . dominates (location , self . dominators) => { } kind @ (LocalKind :: Unused | LocalKind :: SSA (_)) => { * kind = LocalKind :: Memory ; } } , PlaceContext :: MutatingUse (MutatingUseContext :: Store | MutatingUseContext :: Deinit | MutatingUseContext :: SetDiscriminant | MutatingUseContext :: AsmOutput | MutatingUseContext :: Borrow | MutatingUseContext :: RawBorrow | MutatingUseContext :: Projection ,) | PlaceContext :: NonMutatingUse (NonMutatingUseContext :: SharedBorrow | NonMutatingUseContext :: FakeBorrow | NonMutatingUseContext :: RawBorrow | NonMutatingUseContext :: Projection ,) => { self . locals [local] = LocalKind :: Memory ; } PlaceContext :: MutatingUse (MutatingUseContext :: Drop) => { let kind = & mut self . locals [local] ; if * kind != LocalKind :: Memory { let ty = self . fx . mir . local_decls [local] . ty ; let ty = self . fx . monomorphize (ty) ; if self . fx . cx . type_needs_drop (ty) { * kind = LocalKind :: Memory ; } } } PlaceContext :: MutatingUse (MutatingUseContext :: Yield) => bug ! () , } } }
    };
}

impl_447!();