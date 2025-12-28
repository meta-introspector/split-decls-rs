macro_rules! deps {
    () => {
        Producer!();
        IndexedParallelIterator!();
        ProducerCallback!();
        Consumer!();
        ZipProducer!();
        Zip!();
    };
}

macro_rules! impl_1015 {
    () => {
        deps!();
        impl < A , B > IndexedParallelIterator for Zip < A , B > where A : IndexedParallelIterator , B : IndexedParallelIterator , { fn drive < C > (self , consumer : C) -> C :: Result where C : Consumer < Self :: Item > , { bridge (self , consumer) } fn len (& self) -> usize { Ord :: min (self . a . len () , self . b . len ()) } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { return self . a . with_producer (CallbackA { callback , b : self . b , }) ; struct CallbackA < CB , B > { callback : CB , b : B , } impl < CB , ITEM , B > ProducerCallback < ITEM > for CallbackA < CB , B > where B : IndexedParallelIterator , CB : ProducerCallback < (ITEM , B :: Item) > , { type Output = CB :: Output ; fn callback < A > (self , a_producer : A) -> Self :: Output where A : Producer < Item = ITEM > , { self . b . with_producer (CallbackB { a_producer , callback : self . callback , }) } } struct CallbackB < CB , A > { a_producer : A , callback : CB , } impl < CB , A , ITEM > ProducerCallback < ITEM > for CallbackB < CB , A > where A : Producer , CB : ProducerCallback < (A :: Item , ITEM) > , { type Output = CB :: Output ; fn callback < B > (self , b_producer : B) -> Self :: Output where B : Producer < Item = ITEM > , { self . callback . callback (ZipProducer { a : self . a_producer , b : b_producer , }) } } } }
    };
}

impl_1015!()