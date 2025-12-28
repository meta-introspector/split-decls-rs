macro_rules! deps {
    () => {
        Iter!();
        UnindexedProducer!();
        IterParallelProducer!();
        Folder!();
    };
}

macro_rules! impl_762 {
    () => {
        deps!();
        impl < Iter : Iterator + Send > UnindexedProducer for & IterParallelProducer < '_ , Iter > { type Item = Iter :: Item ; fn split (self) -> (Self , Option < Self >) { let update = self . split_count . fetch_update (Ordering :: Relaxed , Ordering :: Relaxed , | c | c . checked_sub (1)) ; (self , update . is_ok () . then_some (self)) } fn fold_with < F > (self , mut folder : F) -> F where F : Folder < Self :: Item > , { if let Some (i) = current_thread_index () { let thread_started = & self . threads_started [i % self . threads_started . len ()] ; if thread_started . swap (true , Ordering :: Relaxed) { return folder ; } } loop { if let Ok (mut iter) = self . iter . lock () { if let Some (it) = iter . next () { drop (iter) ; folder = folder . consume (it) ; if folder . full () { return folder ; } } else { return folder ; } } else { return folder ; } } } }
    };
}

impl_762!();