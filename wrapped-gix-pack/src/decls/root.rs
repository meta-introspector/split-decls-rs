macro_rules! deps {
    () => {
        Tree!();
        EntryRange!();
        ItemSliceSync!();
        Item!();
    };
}

macro_rules! root {
    () => {
        deps!();
        mod root { use crate :: cache :: delta :: { traverse :: util :: ItemSliceSync , Item } ; # [doc = " An item returned by `iter_root_chunks`, allowing access to the `data` stored alongside nodes in a [`Tree`]."] pub (crate) struct Node < 'a , T : Send > { item : & 'a mut Item < T > , child_items : & 'a ItemSliceSync < 'a , Item < T > > , } impl < 'a , T : Send > Node < 'a , T > { # [doc = " SAFETY: `item.children` must uniquely reference elements in child_items that no other currently alive"] # [doc = " item does. All child_items must also have unique children, unless the child_item is itself `item`,"] # [doc = " in which case no other live item should reference it in its `item.children`."] # [doc = ""] # [doc = " This safety invariant can be reliably upheld by making sure `item` comes from a Tree and `child_items`"] # [doc = " was constructed using that Tree's child_items. This works since Tree has this invariant as well: all"] # [doc = " child_items are referenced at most once (really, exactly once) by a node in the tree."] # [doc = ""] # [doc = " Note that this invariant is a bit more relaxed than that on `deltas()`, because this function can be called"] # [doc = " for traversal within a child item, which happens in into_child_iter()"] # [allow (unsafe_code)] pub (super) unsafe fn new (item : & 'a mut Item < T > , child_items : & 'a ItemSliceSync < 'a , Item < T > >) -> Self { Node { item , child_items } } } impl < 'a , T : Send > Node < 'a , T > { # [doc = " Returns the offset into the pack at which the `Node`s data is located."] pub fn offset (& self) -> u64 { self . item . offset } # [doc = " Returns the slice into the data pack at which the pack entry is located."] pub fn entry_slice (& self) -> crate :: data :: EntryRange { self . item . offset .. self . item . next_offset } # [doc = " Returns the node data associated with this node."] pub fn data (& mut self) -> & mut T { & mut self . item . data } # [doc = " Returns true if this node has children, e.g. is not a leaf in the tree."] pub fn has_children (& self) -> bool { ! self . item . children () . is_empty () } # [doc = " Transform this `Node` into an iterator over its children."] # [doc = ""] # [doc = " Children are `Node`s referring to pack entries whose base object is this pack entry."] pub fn into_child_iter (self) -> impl Iterator < Item = Node < 'a , T > > + 'a { let children = self . child_items ; # [allow (unsafe_code)] self . item . children () . iter () . map (move | & index | { let item = unsafe { children . get_mut (index as usize) } ; unsafe { Node :: new (item , children) } }) } } }
    };
}

root!();