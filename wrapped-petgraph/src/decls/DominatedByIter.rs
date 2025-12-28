macro_rules! DominatedByIter {
    () => {
        # [doc = " Iterator for nodes dominated by a given node."] # [derive (Debug , Clone)] pub struct DominatedByIter < 'a , N > where N : 'a + Copy + Eq + Hash , { iter : Iter < 'a , N , N > , node : N , }
    };
}

DominatedByIter!();