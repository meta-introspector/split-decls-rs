macro_rules! deps {
    () => {
        Dominators!();
    };
}

macro_rules! DominatorsIter {
    () => {
        deps!();
        # [doc = " Iterator for a node's dominators."] # [derive (Debug , Clone)] pub struct DominatorsIter < 'a , N > where N : 'a + Copy + Eq + Hash , { dominators : & 'a Dominators < N > , node : Option < N > , }
    };
}

DominatorsIter!();