macro_rules! deps {
    () => {
        FlatMapFolder!();
        UnindexedConsumer!();
        IntoParallelIterator!();
        Folder!();
    };
}

macro_rules! impl_517 {
    () => {
        deps!();
        impl < 'f , T , U , C , F > Folder < T > for FlatMapFolder < 'f , C , F , C :: Result > where C : UnindexedConsumer < U :: Item > , F : Fn (T) -> U + Sync , U : IntoParallelIterator , { type Result = C :: Result ; fn consume (self , item : T) -> Self { let map_op = self . map_op ; let par_iter = map_op (item) . into_par_iter () ; let consumer = self . base . split_off_left () ; let result = par_iter . drive_unindexed (consumer) ; let previous = match self . previous { None => Some (result) , Some (previous) => { let reducer = self . base . to_reducer () ; Some (reducer . reduce (previous , result)) } } ; FlatMapFolder { base : self . base , map_op , previous , } } fn complete (self) -> Self :: Result { match self . previous { Some (previous) => previous , None => self . base . into_folder () . complete () , } } fn full (& self) -> bool { self . base . full () } }
    };
}

impl_517!();