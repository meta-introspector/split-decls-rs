macro_rules! delegate_all {
    () => {
        # [allow (unused_macros)] macro_rules ! delegate_all { ($ ty : ident , $ base : ident) => { delegate_load_store ! ($ ty , $ base) ; delegate_swap ! ($ ty , $ base) ; delegate_cas ! ($ ty , $ base) ; } ; }
    };
}

delegate_all!();