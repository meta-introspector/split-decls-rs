macro_rules! deps {
    () => {
        Decomposition!();
    };
}

macro_rules! impl_111 {
    () => {
        deps!();
        impl < I > Iterator for Decomposition < '_ , I > where I : Iterator < Item = char > , { type Item = char ; fn next (& mut self) -> Option < char > { if let Some (ret) = self . buffer . get (self . buffer_pos) . map (| c | c . character ()) { self . buffer_pos += 1 ; if self . buffer_pos == self . buffer . len () { self . buffer . clear () ; self . buffer_pos = 0 ; } return Some (ret) ; } debug_assert_eq ! (self . buffer_pos , 0) ; let c_and_trie_val = self . pending . take () ? ; Some (self . decomposing_next (c_and_trie_val)) } }
    };
}

impl_111!()