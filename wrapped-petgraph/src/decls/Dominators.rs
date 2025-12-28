macro_rules! Dominators {
    () => {
        # [doc = " The dominance relation for some graph and root."] # [derive (Debug , Clone)] pub struct Dominators < N > where N : Copy + Eq + Hash , { root : N , dominators : HashMap < N , N > , }
    };
}

Dominators!();