macro_rules! deps {
    () => {
        LocalKind!();
        PlaceRef!();
        BuilderMethods!();
        LocalAnalyzer!();
    };
}

macro_rules! impl_446 {
    () => {
        deps!();
        impl < 'a , 'b , 'tcx , Bx : BuilderMethods < 'b , 'tcx > > LocalAnalyzer < 'a , 'b , 'tcx , Bx > { fn define (& mut self , local : mir :: Local , location : DefLocation) { let fx = self . fx ; let kind = & mut self . locals [local] ; let decl = & fx . mir . local_decls [local] ; match * kind { LocalKind :: ZST => { } LocalKind :: Memory => { } LocalKind :: Unused => { let ty = fx . monomorphize (decl . ty) ; let layout = fx . cx . spanned_layout_of (ty , decl . source_info . span) ; * kind = if fx . cx . is_backend_immediate (layout) || fx . cx . is_backend_scalar_pair (layout) { LocalKind :: SSA (location) } else { LocalKind :: Memory } ; } LocalKind :: SSA (_) => * kind = LocalKind :: Memory , } } fn process_place (& mut self , place_ref : & mir :: PlaceRef < 'tcx > , context : PlaceContext , location : Location ,) { if ! place_ref . projection . is_empty () { const COPY_CONTEXT : PlaceContext = PlaceContext :: NonMutatingUse (NonMutatingUseContext :: Copy) ; for elem in place_ref . projection { if let mir :: PlaceElem :: Index (index_local) = * elem { self . visit_local (index_local , COPY_CONTEXT , location) ; } } if self . locals [place_ref . local] == LocalKind :: Memory { return ; } if place_ref . is_indirect_first_projection () { self . visit_local (place_ref . local , COPY_CONTEXT , location) ; return ; } if context . is_mutating_use () { let mut_projection = PlaceContext :: MutatingUse (MutatingUseContext :: Projection) ; self . visit_local (place_ref . local , mut_projection , location) ; return ; } let base_ty = self . fx . monomorphized_place_ty (mir :: PlaceRef :: from (place_ref . local)) ; let mut layout = self . fx . cx . layout_of (base_ty) ; for elem in place_ref . projection { layout = match * elem { mir :: PlaceElem :: Field (fidx , ..) => layout . field (self . fx . cx , fidx . as_usize ()) , mir :: PlaceElem :: Downcast (_ , vidx) if let abi :: Variants :: Single { index : single_variant } = layout . variants && vidx == single_variant => { layout . for_variant (self . fx . cx , vidx) } mir :: PlaceElem :: Subtype (subtype_ty) => { let subtype_ty = self . fx . monomorphize (subtype_ty) ; self . fx . cx . layout_of (subtype_ty) } _ => { self . locals [place_ref . local] = LocalKind :: Memory ; return ; } } } debug_assert ! (! self . fx . cx . is_backend_ref (layout) , "Post-projection {place_ref:?} layout should be non-Ref, but it's {layout:?}" ,) ; } self . visit_local (place_ref . local , context , location) ; } }
    };
}

impl_446!();