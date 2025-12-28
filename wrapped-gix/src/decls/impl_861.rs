macro_rules! deps {
    () => {
        Info!();
        Repository!();
    };
}

macro_rules! impl_861 {
    () => {
        deps!();
        # [doc = " Initialization and detachment"] impl < 'repo > Info < 'repo > { # [doc = " Create a new instance that represents `info`, but is attached to `repo` as well."] pub fn new (info : gix_traverse :: commit :: Info , repo : & 'repo Repository) -> Self { Info { id : info . id , parent_ids : info . parent_ids , commit_time : info . commit_time , repo , } } # [doc = " Consume this instance and remove the reference to the underlying repository."] # [doc = ""] # [doc = " This is useful for sending instances across threads, for example."] pub fn detach (self) -> gix_traverse :: commit :: Info { gix_traverse :: commit :: Info { id : self . id , parent_ids : self . parent_ids , commit_time : self . commit_time , } } }
    };
}

impl_861!();