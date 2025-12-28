macro_rules! deps {
    () => {
        Distribution!();
        StandardUniform!();
        ThreadRng!();
    };
}

macro_rules! random {
    () => {
        deps!();
        # [doc = " Generate a random value using the thread-local random number generator."] # [doc = ""] # [doc = " This function is shorthand for <code>[rng()].[random()](Rng::random)</code>:"] # [doc = ""] # [doc = " -   See [`ThreadRng`] for documentation of the generator and security"] # [doc = " -   See [`StandardUniform`] for documentation of supported types and distributions"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " let x = rand::random::<u8>();"] # [doc = " println!(\"{}\", x);"] # [doc = ""] # [doc = " let y = rand::random::<f64>();"] # [doc = " println!(\"{}\", y);"] # [doc = ""] # [doc = " if rand::random() { // generates a boolean"] # [doc = "     println!(\"Better lucky than good!\");"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " If you're calling `random()` repeatedly, consider using a local `rng`"] # [doc = " handle to save an initialization-check on each usage:"] # [doc = ""] # [doc = " ```"] # [doc = " use rand::Rng; // provides the `random` method"] # [doc = ""] # [doc = " let mut rng = rand::rng(); // a local handle to the generator"] # [doc = ""] # [doc = " let mut v = vec![1, 2, 3];"] # [doc = ""] # [doc = " for x in v.iter_mut() {"] # [doc = "     *x = rng.random();"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " [`StandardUniform`]: distr::StandardUniform"] # [doc = " [`ThreadRng`]: rngs::ThreadRng"] # [cfg (feature = "thread_rng")] # [inline] pub fn random < T > () -> T where StandardUniform : Distribution < T > , { rng () . random () }
    };
}

random!();