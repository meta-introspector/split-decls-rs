macro_rules! assert_panic {
    () => {
        # [allow (unused_unsafe)] # [track_caller] fn assert_panic < T : std :: fmt :: Debug > (f : impl FnOnce () -> T) -> std :: string :: String { let backtrace = std :: env :: var_os ("RUST_BACKTRACE") ; let hook = std :: panic :: take_hook () ; unsafe { std :: env :: set_var ("RUST_BACKTRACE" , "0") } std :: panic :: set_hook (std :: boxed :: Box :: new (| _ | { })) ; let res = std :: panic :: catch_unwind (std :: panic :: AssertUnwindSafe (f)) ; std :: panic :: set_hook (hook) ; match backtrace { Some (v) => unsafe { std :: env :: set_var ("RUST_BACKTRACE" , v) } , None => unsafe { std :: env :: remove_var ("RUST_BACKTRACE") } , } let msg = res . unwrap_err () ; msg . downcast_ref :: < std :: string :: String > () . cloned () . unwrap_or_else (| | msg . downcast_ref :: < & 'static str > () . copied () . unwrap () . into ()) }
    };
}

assert_panic!();