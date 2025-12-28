macro_rules! deps {
    () => {
        DFA!();
        StateID!();
        StateTransitionIter!();
        Unit!();
    };
}

macro_rules! StateSparseTransitionIter {
    () => {
        deps!();
        # [doc = " An iterator over all non-DEAD transitions in a single DFA state using a"] # [doc = " sparse representation."] # [doc = ""] # [doc = " Each transition is represented by a triple. The first two elements of the"] # [doc = " triple comprise an inclusive byte range while the last element corresponds"] # [doc = " to the transition taken for all bytes in the range."] # [doc = ""] # [doc = " As a convenience, this always returns `alphabet::Unit` values of the same"] # [doc = " type. That is, you'll never get a (byte, EOI) or a (EOI, byte). Only (byte,"] # [doc = " byte) and (EOI, EOI) values are yielded."] # [derive (Debug)] pub (crate) struct StateSparseTransitionIter < 'a > { dense : StateTransitionIter < 'a > , cur : Option < (alphabet :: Unit , alphabet :: Unit , StateID) > , }
    };
}

StateSparseTransitionIter!()