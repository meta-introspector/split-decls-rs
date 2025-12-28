macro_rules! RemapHiddenTyRegions {
    () => {
        struct RemapHiddenTyRegions < 'tcx > { tcx : TyCtxt < 'tcx > , # [doc = " Map from early/late params of the impl to identity regions of the RPITIT (GAT)"] # [doc = " in the trait."] map : FxIndexMap < ty :: Region < 'tcx > , ty :: Region < 'tcx > > , num_trait_args : usize , num_impl_args : usize , # [doc = " Def id of the RPITIT (GAT) in the *trait*."] def_id : DefId , # [doc = " Def id of the impl method which owns the opaque hidden type we're remapping."] impl_m_def_id : DefId , # [doc = " The hidden type we're remapping. Useful for diagnostics."] ty : Ty < 'tcx > , # [doc = " Span of the return type. Useful for diagnostics."] return_span : Span , }
    };
}

RemapHiddenTyRegions!()