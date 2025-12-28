macro_rules! deps {
    () => {
        LazyCommit!();
        Graph!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        # [doc = " Lazy commit access"] impl < 'cache , T > Graph < '_ , 'cache , T > { # [doc = " Lookup `id` without failing if the commit doesn't exist or `id` isn't a commit,"] # [doc = " and assure that `id` is inserted into our set"] # [doc = " with a `default` value assigned to it."] # [doc = " `update_data(data)` gets run either on existing or no new data."] # [doc = " Return the commit when done."] # [doc = ""] # [doc = " Note that none of the data updates happen if `id` didn't exist."] # [doc = ""] # [doc = " If only commit data is desired without the need for attaching custom data, use"] # [doc = " [`try_lookup(id)`][Graph::try_lookup()] instead."] pub fn try_lookup_or_insert_default (& mut self , id : gix_hash :: ObjectId , default : impl FnOnce () -> T , update_data : impl FnOnce (& mut T) ,) -> Result < Option < LazyCommit < '_ , 'cache > > , get_or_insert_default :: Error > { let res = try_lookup (& id , & * self . find , self . cache , & mut self . buf) ? ; Ok (res . inspect (| _commit | match self . map . entry (id) { gix_hashtable :: hash_map :: Entry :: Vacant (entry) => { let mut data = default () ; update_data (& mut data) ; entry . insert (data) ; } gix_hashtable :: hash_map :: Entry :: Occupied (mut entry) => { update_data (entry . get_mut ()) ; } })) } # [doc = " Try to lookup `id` and return a handle to it for accessing its data, but don't fail if the commit doesn't exist"] # [doc = " or isn't a commit."] # [doc = ""] # [doc = " It's possible that commits don't exist if the repository is shallow."] pub fn try_lookup (& mut self , id : & gix_hash :: oid ,) -> Result < Option < LazyCommit < '_ , 'cache > > , gix_object :: find :: existing_iter :: Error > { try_lookup (id , & * self . find , self . cache , & mut self . buf) } # [doc = " Lookup `id` and return a handle to it, or fail if it doesn't exist or is no commit."] pub fn lookup (& mut self , id : & gix_hash :: oid ,) -> Result < LazyCommit < '_ , 'cache > , gix_object :: find :: existing_iter :: Error > { self . try_lookup (id) ? . ok_or (gix_object :: find :: existing_iter :: Error :: NotFound { oid : id . to_owned () }) } }
    };
}

impl_16!();