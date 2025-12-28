macro_rules! deps {
    () => {
        Node!();
    };
}

macro_rules! CursorMut {
    () => {
        deps!();
        # [doc = " The `CursorMut` struct and its implementation provide the basic mutable Cursor API for Linked"] # [doc = " lists as proposed in"] # [doc = " [here](https://github.com/rust-lang/rfcs/blob/master/text/2570-linked-list-cursors.md), with"] # [doc = " several exceptions:"] # [doc = ""] # [doc = " - It behaves similarly to Rust's Iterators, returning `None` when the end of the list is"] # [doc = "   reached. A _guard_ node is positioned between the head and tail of the linked list to"] # [doc = "   facilitate this. If the cursor is over this guard node, `None` is returned, signaling the end"] # [doc = "   or start of the list. From this position, the cursor can move in either direction as the"] # [doc = "   linked list is circular, with the guard node connecting the two ends."] # [doc = " - The current implementation does not include an `index` method, as it does not track the index"] # [doc = "   of its elements. It provides access to each map entry as a tuple of `(&K, &mut V)`."] # [doc = ""] pub struct CursorMut < 'a , K , V , S > { cur : * mut Node < K , V > , hash_builder : & 'a S , free : & 'a mut Option < NonNull < Node < K , V > > > , values : & 'a mut Option < NonNull < Node < K , V > > > , table : & 'a mut hashbrown :: HashTable < NonNull < Node < K , V > > > , }
    };
}

CursorMut!()