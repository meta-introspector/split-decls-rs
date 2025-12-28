macro_rules! deps {
    () => {
        Graph!();
        Commit!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        # [doc = " Commit based methods"] impl < T : Default > Graph < '_ , '_ , Commit < T > > { # [doc = " Lookup `id` in the graph, but insert it if it's not yet present by looking it up without failing if the commit doesn't exist."] # [doc = " Newly inserted commits are populated with default data."] # [doc = " `update_data(data)` gets run either on existing or on new data."] # [doc = ""] # [doc = " Note that none of the data updates happen if `id` didn't exist."] # [doc = ""] # [doc = " If only commit data is desired without the need for attaching custom data, use"] # [doc = " [`try_lookup(id).to_owned()`][Graph::try_lookup()] instead."] pub fn get_or_insert_commit (& mut self , id : gix_hash :: ObjectId , update_data : impl FnOnce (& mut T) ,) -> Result < Option < & mut Commit < T > > , get_or_insert_default :: Error > { self . get_or_insert_commit_default (id , T :: default , update_data) } # [doc = " Lookup `id` in the graph, but insert it if it's not yet present by looking it up without failing if the commit doesn't exist."] # [doc = " `update_commit(commit)` gets run either on existing or on new data."] # [doc = ""] # [doc = " Note that none of the data updates happen if `id` didn't exist in the graph."] pub fn get_or_insert_full_commit (& mut self , id : gix_hash :: ObjectId , update_commit : impl FnOnce (& mut Commit < T >) ,) -> Result < Option < & mut Commit < T > > , get_or_insert_default :: Error > { match self . map . entry (id) { gix_hashtable :: hash_map :: Entry :: Vacant (entry) => { let res = try_lookup (& id , & * self . find , self . cache , & mut self . buf) ? ; let commit = match res { None => return Ok (None) , Some (commit) => commit , } ; let mut commit = commit . to_owned (T :: default) ? ; update_commit (& mut commit) ; entry . insert (commit) ; } gix_hashtable :: hash_map :: Entry :: Occupied (mut entry) => { update_commit (entry . get_mut ()) ; } } Ok (self . map . get_mut (& id)) } }
    };
}

impl_15!()