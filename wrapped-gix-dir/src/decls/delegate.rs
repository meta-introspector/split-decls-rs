macro_rules! deps {
    () => {
        Status!();
        Delegate!();
        Action!();
        EntryRef!();
        Entry!();
    };
}

macro_rules! delegate {
    () => {
        deps!();
        # [doc = " Ready-made delegate implementations."] pub mod delegate { use crate :: { entry , walk , walk :: Action , Entry , EntryRef } ; type Entries = Vec < (Entry , Option < entry :: Status >) > ; # [doc = " A [`Delegate`](walk::Delegate) implementation that collects all `entries` along with their directory status, if present."] # [doc = ""] # [doc = " Note that this allocates for each entry."] # [derive (Default)] pub struct Collect { # [doc = " All collected entries, in any order."] pub unorded_entries : Entries , } impl Collect { # [doc = " Return the list of entries that were emitted, sorted ascending by their repository-relative tree path."] pub fn into_entries_by_path (mut self) -> Entries { self . unorded_entries . sort_by (| a , b | a . 0 . rela_path . cmp (& b . 0 . rela_path)) ; self . unorded_entries } } impl walk :: Delegate for Collect { fn emit (& mut self , entry : EntryRef < '_ > , dir_status : Option < entry :: Status >) -> Action { self . unorded_entries . push ((entry . to_owned () , dir_status)) ; walk :: Action :: Continue } } }
    };
}

delegate!();