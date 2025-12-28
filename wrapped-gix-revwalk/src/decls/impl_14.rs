macro_rules! deps {
    () => {
        Commit!();
        Graph!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        # [doc = " Commit based methods"] impl < T > Graph < '_ , '_ , Commit < T > > { # [doc = " Lookup `id` in the graph, but insert it if it's not yet present by looking it up without failing if the commit doesn't exist."] # [doc = " Call `new_data()` to obtain data for a newly inserted commit."] # [doc = " `update_data(data)` gets run either on existing or on new data."] # [doc = ""] # [doc = " Note that none of the data updates happen if `id` didn't exist."] pub fn get_or_insert_commit_default (& mut self , id : gix_hash :: ObjectId , new_data : impl FnOnce () -> T , update_data : impl FnOnce (& mut T) ,) -> Result < Option < & mut Commit < T > > , get_or_insert_default :: Error > { match self . map . entry (id) { gix_hashtable :: hash_map :: Entry :: Vacant (entry) => { let res = try_lookup (& id , & * self . find , self . cache , & mut self . buf) ? ; let commit = match res { None => return Ok (None) , Some (commit) => commit , } ; let mut commit = commit . to_owned (new_data) ? ; update_data (& mut commit . data) ; entry . insert (commit) ; } gix_hashtable :: hash_map :: Entry :: Occupied (mut entry) => { update_data (& mut entry . get_mut () . data) ; } } Ok (self . map . get_mut (& id)) } # [doc = " For each stored commit, call `clear` on its data."] pub fn clear_commit_data (& mut self , mut clear : impl FnMut (& mut T)) { self . map . values_mut () . for_each (| c | clear (& mut c . data)) ; } }
    };
}

impl_14!()