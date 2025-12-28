macro_rules! deps {
    () => {
        DFA!();
    };
}

macro_rules! OwnedDFA {
    () => {
        deps!();
        # [doc = " A convenience alias for an owned DFA. We use this particular instantiation"] # [doc = " a lot in this crate, so it's worth giving it a name. This instantiation"] # [doc = " is commonly used for mutable APIs on the DFA while building it. The main"] # [doc = " reason for making DFAs generic is no_std support, and more generally,"] # [doc = " making it possible to load a DFA from an arbitrary slice of bytes."] # [cfg (feature = "alloc")] pub (crate) type OwnedDFA = DFA < alloc :: vec :: Vec < u32 > > ;
    };
}

OwnedDFA!()