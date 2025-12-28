macro_rules! deps {
    () => {
        DFA!();
    };
}

macro_rules! ByteClasses {
    () => {
        deps!();
        # [doc = " A representation of byte oriented equivalence classes."] # [doc = ""] # [doc = " This is used in a DFA to reduce the size of the transition table. This can"] # [doc = " have a particularly large impact not only on the total size of a dense DFA,"] # [doc = " but also on compile times."] # [doc = ""] # [doc = " The essential idea here is that the alphabet of a DFA is shrunk from the"] # [doc = " usual 256 distinct byte values down to a set of equivalence classes. The"] # [doc = " guarantee you get is that any byte belonging to the same equivalence class"] # [doc = " can be treated as if it were any other byte in the same class, and the"] # [doc = " result of a search wouldn't change."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " This example shows how to get byte classes from an"] # [doc = " [`NFA`](crate::nfa::thompson::NFA) and ask for the class of various bytes."] # [doc = ""] # [doc = " ```"] # [doc = " use regex_automata::nfa::thompson::NFA;"] # [doc = ""] # [doc = " let nfa = NFA::new(\"[a-z]+\")?;"] # [doc = " let classes = nfa.byte_classes();"] # [doc = " // 'a' and 'z' are in the same class for this regex."] # [doc = " assert_eq!(classes.get(b'a'), classes.get(b'z'));"] # [doc = " // But 'a' and 'A' are not."] # [doc = " assert_ne!(classes.get(b'a'), classes.get(b'A'));"] # [doc = ""] # [doc = " # Ok::<(), Box<dyn std::error::Error>>(())"] # [doc = " ```"] # [derive (Clone , Copy)] pub struct ByteClasses ([u8 ; 256]) ;
    };
}

ByteClasses!()