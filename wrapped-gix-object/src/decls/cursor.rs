macro_rules! deps {
    () => {
        UpsertMode!();
        WriteMode!();
        Entry!();
        Cursor!();
        Error!();
        Editor!();
        EntryKind!();
        Tree!();
    };
}

macro_rules! cursor {
    () => {
        deps!();
        mod cursor { use bstr :: { BStr , BString } ; use gix_hash :: ObjectId ; use crate :: { tree , tree :: { editor :: { Cursor , UpsertMode , WriteMode } , Editor , EntryKind , } , Tree , } ; # [doc = " Cursor handling"] impl < 'a > Editor < 'a > { # [doc = " Turn ourselves as a cursor, which points to the same tree as the editor."] # [doc = ""] # [doc = " This is useful if a method takes a [`Cursor`], not an [`Editor`]."] pub fn to_cursor (& mut self) -> Cursor < '_ , 'a > { Cursor { parent : self , prefix : BString :: default () , } } # [doc = " Create a cursor at the given `rela_path`, which must be a tree or is turned into a tree as its own edit."] # [doc = ""] # [doc = " The returned cursor will then allow applying edits to the tree at `rela_path` as root."] # [doc = " If `rela_path` is a single empty string, it is equivalent to using the current instance itself."] pub fn cursor_at < I , C > (& mut self , rela_path : I) -> Result < Cursor < '_ , 'a > , super :: Error > where I : IntoIterator < Item = C > , C : AsRef < BStr > , { self . path_buf . borrow_mut () . clear () ; self . upsert_or_remove_at_pathbuf (rela_path , Some ((EntryKind :: Tree , self . object_hash . null () , UpsertMode :: AssureTreeOnly)) ,) ? ; let prefix = self . path_buf . borrow_mut () . clone () ; Ok (Cursor { prefix , parent : self , }) } } impl Cursor < '_ , '_ > { # [doc = " Obtain the entry at `rela_path` or return `None` if none was found, or the tree wasn't yet written"] # [doc = " to that point."] # [doc = " Note that after [writing](Self::write) only the root path remains, all other intermediate trees are removed."] # [doc = " The entry can be anything that can be stored in a tree, but may have a null-id if it's a newly"] # [doc = " inserted tree. Also, ids of trees might not be accurate as they may have been changed in memory."] pub fn get < I , C > (& self , rela_path : I) -> Option < & tree :: Entry > where I : IntoIterator < Item = C > , C : AsRef < BStr > , { self . parent . path_buf . borrow_mut () . clone_from (& self . prefix) ; self . parent . get_inner (rela_path) } # [doc = " Like [`Editor::upsert()`], but with the constraint of only editing in this cursor's tree."] pub fn upsert < I , C > (& mut self , rela_path : I , kind : EntryKind , id : ObjectId) -> Result < & mut Self , super :: Error > where I : IntoIterator < Item = C > , C : AsRef < BStr > , { self . parent . path_buf . borrow_mut () . clone_from (& self . prefix) ; self . parent . upsert_or_remove_at_pathbuf (rela_path , Some ((kind , id , UpsertMode :: Normal))) ? ; Ok (self) } # [doc = " Like [`Editor::remove()`], but with the constraint of only editing in this cursor's tree."] pub fn remove < I , C > (& mut self , rela_path : I) -> Result < & mut Self , super :: Error > where I : IntoIterator < Item = C > , C : AsRef < BStr > , { self . parent . path_buf . borrow_mut () . clone_from (& self . prefix) ; self . parent . upsert_or_remove_at_pathbuf (rela_path , None) ? ; Ok (self) } # [doc = " Like [`Editor::write()`], but will write only the subtree of the cursor."] pub fn write < E > (& mut self , out : impl FnMut (& Tree) -> Result < ObjectId , E >) -> Result < ObjectId , E > { self . parent . path_buf . borrow_mut () . clone_from (& self . prefix) ; self . parent . write_at_pathbuf (out , WriteMode :: FromCursor) } } }
    };
}

cursor!();