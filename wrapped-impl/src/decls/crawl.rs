macro_rules! deps {
    () => {
        ParamsInScope!();
    };
}

macro_rules! crawl {
    () => {
        deps!();
        fn crawl (in_scope : & ParamsInScope , ty : & Type , found : & mut bool) { if let Type :: Path (ty) = ty { if let Some (qself) = & ty . qself { crawl (in_scope , & qself . ty , found) ; } else { let front = ty . path . segments . first () . unwrap () ; if front . arguments . is_none () && in_scope . names . contains (& front . ident) { * found = true ; } } for segment in & ty . path . segments { if let PathArguments :: AngleBracketed (arguments) = & segment . arguments { for arg in & arguments . args { if let GenericArgument :: Type (ty) = arg { crawl (in_scope , ty , found) ; } } } } } }
    };
}

crawl!();