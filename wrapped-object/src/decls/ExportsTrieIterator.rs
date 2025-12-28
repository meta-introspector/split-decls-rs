macro_rules! deps {
    () => {
        NodeIterator!();
    };
}

macro_rules! ExportsTrieIterator {
    () => {
        deps!();
        # [doc = " Iterator over the exports trie."] # [derive (Debug)] pub struct ExportsTrieIterator < 'data > { node_iter : NodeIterator < 'data > , }
    };
}

ExportsTrieIterator!();