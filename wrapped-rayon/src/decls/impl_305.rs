macro_rules! deps {
    () => {
        Producer!();
        ProducerCallback!();
        IndexedParallelIterator!();
        Chain!();
        ChainProducer!();
        Consumer!();
    };
}

macro_rules! impl_305 {
    () => {
        deps!();
        impl < A , B > IndexedParallelIterator for Chain < A , B > where A : IndexedParallelIterator , B : IndexedParallelIterator < Item = A :: Item > , { fn drive < C > (self , consumer : C) -> C :: Result where C : Consumer < Self :: Item > , { let Chain { a , b } = self ; let (left , right , reducer) = consumer . split_at (a . len ()) ; let (a , b) = join (| | a . drive (left) , | | b . drive (right)) ; reducer . reduce (a , b) } fn len (& self) -> usize { self . a . len () . checked_add (self . b . len ()) . expect ("overflow") } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { let a_len = self . a . len () ; return self . a . with_producer (CallbackA { callback , a_len , b : self . b , }) ; struct CallbackA < CB , B > { callback : CB , a_len : usize , b : B , } impl < CB , B > ProducerCallback < B :: Item > for CallbackA < CB , B > where B : IndexedParallelIterator , CB : ProducerCallback < B :: Item > , { type Output = CB :: Output ; fn callback < A > (self , a_producer : A) -> Self :: Output where A : Producer < Item = B :: Item > , { self . b . with_producer (CallbackB { callback : self . callback , a_len : self . a_len , a_producer , }) } } struct CallbackB < CB , A > { callback : CB , a_len : usize , a_producer : A , } impl < CB , A > ProducerCallback < A :: Item > for CallbackB < CB , A > where A : Producer , CB : ProducerCallback < A :: Item > , { type Output = CB :: Output ; fn callback < B > (self , b_producer : B) -> Self :: Output where B : Producer < Item = A :: Item > , { let producer = ChainProducer :: new (self . a_len , self . a_producer , b_producer) ; self . callback . callback (producer) } } } }
    };
}

impl_305!();