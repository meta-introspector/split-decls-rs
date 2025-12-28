macro_rules! i {
    () => {
        # [cfg (debug_assertions)] macro_rules ! i { ($ array : expr , $ index : expr) => { *$ array . get ($ index) . unwrap () } ; ($ array : expr , $ index : expr , = , $ rhs : expr) => { *$ array . get_mut ($ index) . unwrap () = $ rhs ; } ; ($ array : expr , $ index : expr , -= , $ rhs : expr) => { *$ array . get_mut ($ index) . unwrap () -= $ rhs ; } ; ($ array : expr , $ index : expr , += , $ rhs : expr) => { *$ array . get_mut ($ index) . unwrap () += $ rhs ; } ; ($ array : expr , $ index : expr , &= , $ rhs : expr) => { *$ array . get_mut ($ index) . unwrap () &= $ rhs ; } ; ($ array : expr , $ index : expr , == , $ rhs : expr) => { *$ array . get_mut ($ index) . unwrap () == $ rhs } ; }
    };
}

i!();