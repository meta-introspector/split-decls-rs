macro_rules! deps {
    () => {
        Position!();
        File!();
        Graph!();
        Commit!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        # [doc = " Access"] impl Graph { # [doc = " Returns the commit at the given position `pos`."] # [doc = ""] # [doc = " # Panics"] # [doc = " If `pos` is greater or equal to [`num_commits()`][Graph::num_commits()]."] pub fn commit_at (& self , pos : Position) -> Commit < '_ > { let r = self . lookup_by_pos (pos) ; r . file . commit_at (r . pos) } # [doc = " Returns the commit matching the given `id`."] pub fn commit_by_id (& self , id : impl AsRef < gix_hash :: oid >) -> Option < Commit < '_ > > { let r = self . lookup_by_id (id . as_ref ()) ? ; Some (r . file . commit_at (r . file_pos)) } # [doc = " Returns the `hash` at the given position `pos`."] # [doc = ""] # [doc = " # Panics"] # [doc = " If `pos` is greater or equal to [`num_commits()`][Graph::num_commits()]."] pub fn id_at (& self , pos : Position) -> & gix_hash :: oid { let r = self . lookup_by_pos (pos) ; r . file . id_at (r . pos) } # [doc = " Iterate over commits in unsorted order."] pub fn iter_commits (& self) -> impl Iterator < Item = Commit < '_ > > { self . files . iter () . flat_map (File :: iter_commits) } # [doc = " Iterate over commit IDs in unsorted order."] pub fn iter_ids (& self) -> impl Iterator < Item = & gix_hash :: oid > { self . files . iter () . flat_map (File :: iter_ids) } # [doc = " Translate the given `id` to its position in the file."] pub fn lookup (& self , id : impl AsRef < gix_hash :: oid >) -> Option < Position > { Some (self . lookup_by_id (id . as_ref ()) ? . graph_pos) } # [doc = " Returns the number of commits stored in this file."] pub fn num_commits (& self) -> u32 { self . files . iter () . map (File :: num_commits) . sum () } }
    };
}

impl_3!();