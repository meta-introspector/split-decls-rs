macro_rules! deps {
    () => {
        Machine!();
        GenericMachine!();
    };
}

macro_rules! dispatch_light256 {
    () => {
        deps!();
        # [macro_export] macro_rules ! dispatch_light256 { ($ mach : ident , $ MTy : ident , { $ ([$ pub : tt $ (($ krate : tt)) *]) * fn $ name : ident ($ ($ arg : ident : $ argty : ty) ,* $ (,) *) -> $ ret : ty $ body : block }) => { # [inline (always)] $ ($ pub $ (($ krate)) *) * fn $ name ($ ($ arg : $ argty) ,*) -> $ ret { let $ mach = unsafe { $ crate :: generic :: GenericMachine :: instance () } ; # [inline (always)] fn fn_impl <$ MTy : $ crate :: Machine > ($ mach : $ MTy , $ ($ arg : $ argty) ,*) -> $ ret $ body fn_impl ($ mach , $ ($ arg) ,*) } } ; ($ mach : ident , $ MTy : ident , { $ ([$ pub : tt $ (($ krate : tt)) *]) * fn $ name : ident ($ ($ arg : ident : $ argty : ty) ,* $ (,) *) $ body : block }) => { dispatch ! ($ mach , $ MTy , { $ ([$ pub $ (($ krate)) *]) * fn $ name ($ ($ arg : $ argty) ,*) -> () $ body }) ; } }
    };
}

dispatch_light256!()