macro_rules! choose_multiple {
    () => {
        # [doc = " Collects `amount` values at random from the iterable into a vector."] pub fn choose_multiple < I : IntoIterator > (source : I , amount : usize) -> Vec < I :: Item > { with_rng (| rng | rng . choose_multiple (source , amount)) }
    };
}

choose_multiple!()