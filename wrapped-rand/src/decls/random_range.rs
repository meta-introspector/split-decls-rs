macro_rules! deps {
    () => {
        SampleRange!();
        SampleUniform!();
    };
}

macro_rules! random_range {
    () => {
        deps!();
        # [doc = " Generate a random value in the given range using the thread-local random number generator."] # [doc = ""] # [doc = " This function is shorthand for"] # [doc = " <code>[rng()].[random_range](Rng::random_range)(<var>range</var>)</code>."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " let y: f32 = rand::random_range(0.0..=1e9);"] # [doc = " println!(\"{}\", y);"] # [doc = ""] # [doc = " let words: Vec<&str> = \"Mary had a little lamb\".split(' ').collect();"] # [doc = " println!(\"{}\", words[rand::random_range(..words.len())]);"] # [doc = " ```"] # [doc = " Note that the first example can also be achieved (without `collect`'ing"] # [doc = " to a `Vec`) using [`seq::IteratorRandom::choose`]."] # [cfg (feature = "thread_rng")] # [inline] pub fn random_range < T , R > (range : R) -> T where T : distr :: uniform :: SampleUniform , R : distr :: uniform :: SampleRange < T > , { rng () . random_range (range) }
    };
}

random_range!();