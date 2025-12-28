macro_rules! deps {
    () => {
        Walker!();
        WalkerIter!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl < W , C > Iterator for WalkerIter < W , C > where W : Walker < C > , C : Clone , { type Item = W :: Item ; fn next (& mut self) -> Option < Self :: Item > { self . walker . walk_next (self . context . clone ()) } }
    };
}

impl_57!();