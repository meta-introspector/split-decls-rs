macro_rules! deps {
    () => {
        IndexedParallelIterator!();
        ProducerCallback!();
        Iter!();
        ParallelIterator!();
        IntoParallelRefIterator!();
        MultiZip!();
        IntoParallelIterator!();
        UnindexedConsumer!();
        Consumer!();
        IntoParallelRefMutIterator!();
    };
}

macro_rules! multizip_impls {
    () => {
        deps!();
        macro_rules ! multizip_impls { ($ ($ Tuple : ident { $ (($ idx : tt) -> $ T : ident) + }) +) => { $ (impl <$ ($ T ,) +> IntoParallelIterator for ($ ($ T ,) +) where $ ($ T : IntoParallelIterator < Iter : IndexedParallelIterator >,) + { type Item = ($ ($ T :: Item ,) +) ; type Iter = MultiZip < ($ ($ T :: Iter ,) +) >; fn into_par_iter (self) -> Self :: Iter { MultiZip { tuple : ($ (self .$ idx . into_par_iter () ,) +) , } } } impl <'a , $ ($ T ,) +> IntoParallelIterator for &'a ($ ($ T ,) +) where $ ($ T : IntoParallelRefIterator <'a , Iter : IndexedParallelIterator >,) + { type Item = ($ ($ T :: Item ,) +) ; type Iter = MultiZip < ($ ($ T :: Iter ,) +) >; fn into_par_iter (self) -> Self :: Iter { MultiZip { tuple : ($ (self .$ idx . par_iter () ,) +) , } } } impl <'a , $ ($ T ,) +> IntoParallelIterator for &'a mut ($ ($ T ,) +) where $ ($ T : IntoParallelRefMutIterator <'a , Iter : IndexedParallelIterator >,) + { type Item = ($ ($ T :: Item ,) +) ; type Iter = MultiZip < ($ ($ T :: Iter ,) +) >; fn into_par_iter (self) -> Self :: Iter { MultiZip { tuple : ($ (self .$ idx . par_iter_mut () ,) +) , } } } impl <$ ($ T ,) +> ParallelIterator for MultiZip < ($ ($ T ,) +) > where $ ($ T : IndexedParallelIterator ,) + { type Item = ($ ($ T :: Item ,) +) ; fn drive_unindexed < CONSUMER > (self , consumer : CONSUMER) -> CONSUMER :: Result where CONSUMER : UnindexedConsumer < Self :: Item >, { self . drive (consumer) } fn opt_len (& self) -> Option < usize > { Some (self . len ()) } } impl <$ ($ T ,) +> IndexedParallelIterator for MultiZip < ($ ($ T ,) +) > where $ ($ T : IndexedParallelIterator ,) + { fn drive < CONSUMER > (self , consumer : CONSUMER) -> CONSUMER :: Result where CONSUMER : Consumer < Self :: Item >, { reduce ! ($ (self . tuple .$ idx) ,+ => IndexedParallelIterator :: zip) . map (flatten ! ($ ($ T) ,+)) . drive (consumer) } fn len (& self) -> usize { reduce ! ($ (self . tuple .$ idx . len ()) ,+ => Ord :: min) } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item >, { reduce ! ($ (self . tuple .$ idx) ,+ => IndexedParallelIterator :: zip) . map (flatten ! ($ ($ T) ,+)) . with_producer (callback) } }) + } }
    };
}

multizip_impls!();