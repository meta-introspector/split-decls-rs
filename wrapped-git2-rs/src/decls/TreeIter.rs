macro_rules! deps {
    () => {
        Tree!();
    };
}

macro_rules! TreeIter {
    () => {
        deps!();
        # [doc = " An iterator over the entries in a tree."] pub struct TreeIter < 'tree > { range : Range < usize > , tree : & 'tree Tree < 'tree > , }
    };
}

TreeIter!();