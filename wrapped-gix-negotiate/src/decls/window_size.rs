macro_rules! window_size {
    () => {
        # [doc = " Calculate how many `HAVE` lines we may send in one round, with variation depending on whether the `transport_is_stateless` or not."] # [doc = " `window_size` is the previous (or initial) value of the window size."] pub fn window_size (transport_is_stateless : bool , window_size : Option < usize >) -> usize { let current_size = match window_size { None => return 16 , Some (cs) => cs , } ; const PIPESAFE_FLUSH : usize = 32 ; const LARGE_FLUSH : usize = 16384 ; if transport_is_stateless { if current_size < LARGE_FLUSH { current_size * 2 } else { current_size * 11 / 10 } } else if current_size < PIPESAFE_FLUSH { current_size * 2 } else { current_size + PIPESAFE_FLUSH } }
    };
}

window_size!()