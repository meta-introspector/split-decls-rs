macro_rules! span_mirbug {
    () => {
        macro_rules ! span_mirbug { ($ context : expr , $ elem : expr , $ ($ message : tt) *) => ({ $ crate :: type_check :: mirbug ($ context . tcx () , $ context . last_span , format ! ("broken MIR in {:?} ({:?}): {}" , $ context . body () . source . def_id () , $ elem , format_args ! ($ ($ message) *) ,) ,) }) }
    };
}

span_mirbug!();