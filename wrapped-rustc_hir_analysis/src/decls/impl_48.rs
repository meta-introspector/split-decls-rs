macro_rules! deps {
    () => {
        RemapHiddenTyRegions!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl < 'tcx > ty :: FallibleTypeFolder < TyCtxt < 'tcx > > for RemapHiddenTyRegions < 'tcx > { type Error = ErrorGuaranteed ; fn cx (& self) -> TyCtxt < 'tcx > { self . tcx } fn try_fold_region (& mut self , region : ty :: Region < 'tcx > ,) -> Result < ty :: Region < 'tcx > , Self :: Error > { match region . kind () { ty :: ReBound (..) | ty :: ReStatic | ty :: ReError (_) => return Ok (region) , ty :: ReLateParam (_) => { } ty :: ReEarlyParam (ebr) => { if ebr . index as usize >= self . num_impl_args { } else { return Ok (region) ; } } ty :: ReVar (_) | ty :: RePlaceholder (_) | ty :: ReErased => unreachable ! ("should not have leaked vars or placeholders into hidden type of RPITIT") , } let e = if let Some (id_region) = self . map . get (& region) { if let ty :: ReEarlyParam (e) = id_region . kind () { e } else { bug ! ("expected to map region {region} to early-bound identity region, but got {id_region}") ; } } else { let guar = match region . opt_param_def_id (self . tcx , self . impl_m_def_id) { Some (def_id) => { let return_span = if let ty :: Alias (ty :: Opaque , opaque_ty) = self . ty . kind () { self . tcx . def_span (opaque_ty . def_id) } else { self . return_span } ; self . tcx . dcx () . struct_span_err (return_span , "return type captures more lifetimes than trait definition" ,) . with_span_label (self . tcx . def_span (def_id) , "this lifetime was captured") . with_span_note (self . tcx . def_span (self . def_id) , "hidden type must only reference lifetimes captured by this impl trait" ,) . with_note (format ! ("hidden type inferred to be `{}`" , self . ty)) . emit () } None => { self . tcx . dcx () . bug ("should've been able to remap region") ; } } ; return Err (guar) ; } ; Ok (ty :: Region :: new_early_param (self . tcx , ty :: EarlyParamRegion { name : e . name , index : (e . index as usize - self . num_trait_args + self . num_impl_args) as u32 , } ,)) } }
    };
}

impl_48!()