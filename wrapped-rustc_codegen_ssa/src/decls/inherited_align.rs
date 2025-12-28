macro_rules! inherited_align {
    () => {
        # [doc = " If the provided DefId is a method in a trait impl, return the value of the `#[align]`"] # [doc = " attribute on the method prototype (if any)."] fn inherited_align < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : DefId) -> Option < Align > { tcx . codegen_fn_attrs (tcx . trait_item_of (def_id) ?) . alignment }
    };
}

inherited_align!()