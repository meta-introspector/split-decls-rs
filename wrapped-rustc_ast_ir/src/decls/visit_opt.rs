macro_rules! visit_opt {
    () => {
        # [macro_export] macro_rules ! visit_opt { ($ visitor : expr , $ method : ident , $ opt : expr $ (, $ ($ extra_args : expr) ,*) ?) => { if let Some (x) = $ opt { $ crate :: try_visit ! ($ visitor .$ method (x $ (, $ ($ extra_args ,) *) ?)) ; } } }
    };
}

visit_opt!();