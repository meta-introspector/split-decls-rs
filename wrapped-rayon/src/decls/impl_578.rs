macro_rules! deps {
    () => {
        FoldChunksWith!();
        ChunkProducer!();
        Producer!();
        Consumer!();
        IndexedParallelIterator!();
        IntoIter!();
        ProducerCallback!();
    };
}

macro_rules! impl_578 {
    () => {
        deps!();
        impl < I , U , F > IndexedParallelIterator for FoldChunksWith < I , U , F > where I : IndexedParallelIterator , U : Send + Clone , F : Fn (U , I :: Item) -> U + Send + Sync , { fn len (& self) -> usize { self . base . len () . div_ceil (self . chunk_size) } fn drive < C > (self , consumer : C) -> C :: Result where C : Consumer < Self :: Item > , { bridge (self , consumer) } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { let len = self . base . len () ; return self . base . with_producer (Callback { chunk_size : self . chunk_size , len , item : self . item , fold_op : self . fold_op , callback , }) ; struct Callback < CB , T , F > { chunk_size : usize , len : usize , item : T , fold_op : F , callback : CB , } impl < T , U , F , CB > ProducerCallback < T > for Callback < CB , U , F > where CB : ProducerCallback < U > , U : Send + Clone , F : Fn (U , T) -> U + Send + Sync , { type Output = CB :: Output ; fn callback < P > (self , base : P) -> CB :: Output where P : Producer < Item = T > , { let item = self . item ; let fold_op = & self . fold_op ; let fold_iter = move | iter : P :: IntoIter | iter . fold (item . clone () , fold_op) ; let producer = ChunkProducer :: new (self . chunk_size , self . len , base , fold_iter) ; self . callback . callback (producer) } } } }
    };
}

impl_578!();