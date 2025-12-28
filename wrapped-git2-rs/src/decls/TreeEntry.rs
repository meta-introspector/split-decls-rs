macro_rules! TreeEntry {
    () => {
        # [doc = " A structure representing an entry inside of a tree. An entry is borrowed"] # [doc = " from a tree."] pub struct TreeEntry < 'tree > { raw : * mut raw :: git_tree_entry , owned : bool , _marker : marker :: PhantomData < & 'tree raw :: git_tree_entry > , }
    };
}

TreeEntry!()