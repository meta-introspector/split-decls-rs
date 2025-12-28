macro_rules! walk_list {
    () => {
        # [macro_export] macro_rules ! walk_list { ($ visitor : expr , $ method : ident , $ list : expr $ (, $ ($ extra_args : expr) ,*) ?) => { for elem in $ list { $ crate :: try_visit ! ($ visitor .$ method (elem $ (, $ ($ extra_args ,) *) ?)) ; } } }
    };
}

walk_list!();