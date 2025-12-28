macro_rules! deps {
    () => {
        IntoParallelIterator!();
        UnindexedConsumer!();
        FlattenFolder!();
        Folder!();
    };
}

macro_rules! impl_538 {
    () => {
        deps!();
        impl < T , C > Folder < T > for FlattenFolder < C , C :: Result > where C : UnindexedConsumer < T :: Item > , T : IntoParallelIterator , { type Result = C :: Result ; fn consume (self , item : T) -> Self { let par_iter = item . into_par_iter () ; let consumer = self . base . split_off_left () ; let result = par_iter . drive_unindexed (consumer) ; let previous = match self . previous { None => Some (result) , Some (previous) => { let reducer = self . base . to_reducer () ; Some (reducer . reduce (previous , result)) } } ; FlattenFolder { base : self . base , previous , } } fn complete (self) -> Self :: Result { match self . previous { Some (previous) => previous , None => self . base . into_folder () . complete () , } } fn full (& self) -> bool { self . base . full () } }
    };
}

impl_538!()