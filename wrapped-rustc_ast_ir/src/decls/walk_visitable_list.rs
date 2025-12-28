macro_rules! walk_visitable_list {
    () => {
        # [macro_export] macro_rules ! walk_visitable_list { ($ visitor : expr , $ list : expr $ (, $ ($ extra_args : expr) ,*) ?) => { for elem in $ list { $ crate :: try_visit ! (elem . visit_with ($ visitor $ (, $ ($ extra_args ,) *) ?)) ; } } }
    };
}

walk_visitable_list!();