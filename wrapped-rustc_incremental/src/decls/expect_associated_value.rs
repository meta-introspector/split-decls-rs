macro_rules! deps {
    () => {
        AssociatedValueExpectedFor!();
        AssociatedValueExpected!();
    };
}

macro_rules! expect_associated_value {
    () => {
        deps!();
        fn expect_associated_value (tcx : TyCtxt < '_ > , item : & MetaItemInner) -> Symbol { if let Some (value) = item . value_str () { value } else if let Some (ident) = item . ident () { tcx . dcx () . emit_fatal (errors :: AssociatedValueExpectedFor { span : item . span () , ident }) ; } else { tcx . dcx () . emit_fatal (errors :: AssociatedValueExpected { span : item . span () }) ; } }
    };
}

expect_associated_value!();