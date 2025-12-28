macro_rules! deps {
    () => {
        Producer!();
        DropCounter!();
        ProducerCallback!();
    };
}

macro_rules! check_drops {
    () => {
        deps!();
        # [test] fn check_drops () { use std :: sync :: atomic :: { AtomicUsize , Ordering } ; let c = AtomicUsize :: new (0) ; let a = vec ! [DropCounter (& c) ; 10] ; let mut b = vec ! [] ; a . clone () . into_par_iter () . collect_into_vec (& mut b) ; assert_eq ! (c . load (Ordering :: Relaxed) , 0) ; b . into_par_iter () ; assert_eq ! (c . load (Ordering :: Relaxed) , 10) ; a . into_par_iter () . with_producer (Partial) ; assert_eq ! (c . load (Ordering :: Relaxed) , 20) ; # [derive (Clone)] struct DropCounter < 'a > (& 'a AtomicUsize) ; impl < 'a > Drop for DropCounter < 'a > { fn drop (& mut self) { self . 0 . fetch_add (1 , Ordering :: Relaxed) ; } } struct Partial ; impl < 'a > ProducerCallback < DropCounter < 'a > > for Partial { type Output = () ; fn callback < P > (self , producer : P) where P : Producer < Item = DropCounter < 'a > > , { let (a , _) = producer . split_at (5) ; a . into_iter () . next () ; } } }
    };
}

check_drops!();