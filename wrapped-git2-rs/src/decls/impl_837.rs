macro_rules! deps {
    () => {
        TreeEntry!();
        TreeIter!();
    };
}

macro_rules! impl_837 {
    () => {
        deps!();
        impl < 'tree > DoubleEndedIterator for TreeIter < 'tree > { fn next_back (& mut self) -> Option < TreeEntry < 'tree > > { self . range . next_back () . and_then (| i | self . tree . get (i)) } }
    };
}

impl_837!();