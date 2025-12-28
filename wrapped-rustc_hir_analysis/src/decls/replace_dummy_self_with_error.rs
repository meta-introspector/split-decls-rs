macro_rules! replace_dummy_self_with_error {
    () => {
        fn replace_dummy_self_with_error < 'tcx , T : TypeFoldable < TyCtxt < 'tcx > > > (tcx : TyCtxt < 'tcx > , t : T , guar : ErrorGuaranteed ,) -> T { t . fold_with (& mut BottomUpFolder { tcx , ty_op : | ty | { if ty == tcx . types . trait_object_dummy_self { Ty :: new_error (tcx , guar) } else { ty } } , lt_op : | lt | lt , ct_op : | ct | ct , }) }
    };
}

replace_dummy_self_with_error!()