macro_rules! deps {
    () => {
        Graph!();
        LazyCommit!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl < 'cache , T : Default > Graph < '_ , 'cache , T > { # [doc = " Lookup `id` without failing if the commit doesn't exist, and assure that `id` is inserted into our set."] # [doc = " If it wasn't, associate it with the default value. Assure `update_data(data)` gets run."] # [doc = " Return the commit when done."] # [doc = " Note that none of the data updates happen if there was no commit named `id`."] pub fn try_lookup_or_insert (& mut self , id : gix_hash :: ObjectId , update_data : impl FnOnce (& mut T) ,) -> Result < Option < LazyCommit < '_ , 'cache > > , get_or_insert_default :: Error > { self . try_lookup_or_insert_default (id , T :: default , update_data) } }
    };
}

impl_11!()