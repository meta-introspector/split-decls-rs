macro_rules! deps {
    () => {
        Atomic!();
    };
}

macro_rules! Entry {
    () => {
        deps!();
        # [doc = " An entry in a linked list."] # [doc = ""] # [doc = " An Entry is accessed from multiple threads, so it would be beneficial to put it in a different"] # [doc = " cache-line than thread-local data in terms of performance."] # [derive (Debug)] pub (crate) struct Entry { # [doc = " The next entry in the linked list."] # [doc = " If the tag is 1, this entry is marked as deleted."] next : Atomic < Entry > , }
    };
}

Entry!();