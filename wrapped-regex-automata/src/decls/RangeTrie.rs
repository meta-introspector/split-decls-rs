macro_rules! deps {
    () => {
        NextInsert!();
        NextIter!();
        NextDupe!();
        State!();
    };
}

macro_rules! RangeTrie {
    () => {
        deps!();
        # [doc = " A range trie represents an ordered set of sequences of bytes."] # [doc = ""] # [doc = " A range trie accepts as input a sequence of byte ranges and merges"] # [doc = " them into the existing set such that the trie can produce a sorted"] # [doc = " non-overlapping sequence of byte ranges. The sequence emitted corresponds"] # [doc = " precisely to the sequence of bytes matched by the given keys, although the"] # [doc = " byte ranges themselves may be split at different boundaries."] # [doc = ""] # [doc = " The order complexity of this data structure seems difficult to analyze."] # [doc = " If the size of a byte is held as a constant, then insertion is clearly"] # [doc = " O(n) where n is the number of byte ranges in the input key. However, if"] # [doc = " k=256 is our alphabet size, then insertion could be O(k^2 * n). In"] # [doc = " particular it seems possible for pathological inputs to cause insertion"] # [doc = " to do a lot of work. However, for what we use this data structure for,"] # [doc = " there should be no pathological inputs since the ultimate source is always"] # [doc = " a sorted set of Unicode scalar value ranges."] # [doc = ""] # [doc = " Internally, this trie is setup like a finite state machine. Note though"] # [doc = " that it is acyclic."] # [derive (Clone)] pub struct RangeTrie { # [doc = " The states in this trie. The first is always the shared final state."] # [doc = " The second is always the root state. Otherwise, there is no"] # [doc = " particular order."] states : Vec < State > , # [doc = " A free-list of states. When a range trie is cleared, all of its states"] # [doc = " are added to this list. Creating a new state reuses states from this"] # [doc = " list before allocating a new one."] free : Vec < State > , # [doc = " A stack for traversing this trie to yield sequences of byte ranges in"] # [doc = " lexicographic order."] iter_stack : RefCell < Vec < NextIter > > , # [doc = " A buffer that stores the current sequence during iteration."] iter_ranges : RefCell < Vec < Utf8Range > > , # [doc = " A stack used for traversing the trie in order to (deeply) duplicate"] # [doc = " a state. States are recursively duplicated when ranges are split."] dupe_stack : Vec < NextDupe > , # [doc = " A stack used for traversing the trie during insertion of a new"] # [doc = " sequence of byte ranges."] insert_stack : Vec < NextInsert > , }
    };
}

RangeTrie!();