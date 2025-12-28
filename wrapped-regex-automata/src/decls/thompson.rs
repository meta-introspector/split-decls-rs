macro_rules! thompson {
    () => {
        # [cfg (feature = "nfa-thompson")] pub mod thompson ;
    };
}

thompson!()