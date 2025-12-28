macro_rules! deps {
    () => {
        IndexedParallelIterator!();
        Consumer!();
        Producer!();
        ProducerCallback!();
        IntoIter!();
        FoldChunks!();
        ChunkProducer!();
    };
}

macro_rules! impl_571 {
    () => {
        deps!();
        impl < I , ID , U , F > IndexedParallelIterator for FoldChunks < I , ID , F > where I : IndexedParallelIterator , ID : Fn () -> U + Send + Sync , F : Fn (U , I :: Item) -> U + Send + Sync , U : Send , { fn len (& self) -> usize { self . base . len () . div_ceil (self . chunk_size) } fn drive < C > (self , consumer : C) -> C :: Result where C : Consumer < Self :: Item > , { bridge (self , consumer) } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { let len = self . base . len () ; return self . base . with_producer (Callback { chunk_size : self . chunk_size , len , identity : self . identity , fold_op : self . fold_op , callback , }) ; struct Callback < CB , ID , F > { chunk_size : usize , len : usize , identity : ID , fold_op : F , callback : CB , } impl < T , CB , ID , U , F > ProducerCallback < T > for Callback < CB , ID , F > where CB : ProducerCallback < U > , ID : Fn () -> U + Send + Sync , F : Fn (U , T) -> U + Send + Sync , { type Output = CB :: Output ; fn callback < P > (self , base : P) -> CB :: Output where P : Producer < Item = T > , { let identity = & self . identity ; let fold_op = & self . fold_op ; let fold_iter = move | iter : P :: IntoIter | iter . fold (identity () , fold_op) ; let producer = ChunkProducer :: new (self . chunk_size , self . len , base , fold_iter) ; self . callback . callback (producer) } } } }
    };
}

impl_571!();