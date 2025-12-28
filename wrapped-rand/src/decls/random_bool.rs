macro_rules! random_bool {
    () => {
        # [doc = " Return a bool with a probability `p` of being true."] # [doc = ""] # [doc = " This function is shorthand for"] # [doc = " <code>[rng()].[random_bool](Rng::random_bool)(<var>p</var>)</code>."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " println!(\"{}\", rand::random_bool(1.0 / 3.0));"] # [doc = " ```"] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " If `p < 0` or `p > 1`."] # [cfg (feature = "thread_rng")] # [inline] # [track_caller] pub fn random_bool (p : f64) -> bool { rng () . random_bool (p) }
    };
}

random_bool!()