macro_rules! mirbug {
    () => {
        # [track_caller] fn mirbug (tcx : TyCtxt < '_ > , span : Span , msg : String) { tcx . dcx () . span_delayed_bug (span , msg) ; }
    };
}

mirbug!();