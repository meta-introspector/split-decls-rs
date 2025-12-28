macro_rules! deps {
    () => {
        Error!();
        Editor!();
        ToComponents!();
        Cursor!();
    };
}

macro_rules! impl_199 {
    () => {
        deps!();
        # [doc = " Cursor Handling"] impl < 'repo > super :: Editor < 'repo > { # [doc = " Turn ourselves as a cursor, which points to the same tree as the editor."] # [doc = ""] # [doc = " This is useful if a method takes a [`Cursor`], not an [`Editor`](super::Editor)."] pub fn to_cursor (& mut self) -> Cursor < '_ , 'repo > { Cursor { inner : self . inner . to_cursor () , validate : self . validate , repo : self . repo , } } # [doc = " Create a cursor at the given `rela_path`, which must be a tree or is turned into a tree as its own edit."] # [doc = ""] # [doc = " The returned cursor will then allow applying edits to the tree at `rela_path` as root."] # [doc = " If `rela_path` is a single empty string, it is equivalent to using the current instance itself."] pub fn cursor_at (& mut self , rela_path : impl ToComponents ,) -> Result < Cursor < '_ , 'repo > , gix_object :: tree :: editor :: Error > { Ok (Cursor { inner : self . inner . cursor_at (rela_path . to_components ()) ? , validate : self . validate , repo : self . repo , }) } }
    };
}

impl_199!();