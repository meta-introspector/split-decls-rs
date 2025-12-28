macro_rules! nfa {
    () => {
        # [cfg (feature = "nfa-thompson")] pub mod nfa ;
    };
}

nfa!();