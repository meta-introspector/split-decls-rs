macro_rules! upgrade_success_ordering {
    () => {
        # [allow (dead_code)] # [inline] pub (crate) fn upgrade_success_ordering (success : Ordering , failure : Ordering) -> Ordering { match (success , failure) { (Ordering :: Relaxed , Ordering :: Acquire) => Ordering :: Acquire , (Ordering :: Release , Ordering :: Acquire) => Ordering :: AcqRel , (_ , Ordering :: SeqCst) => Ordering :: SeqCst , _ => success , } }
    };
}

upgrade_success_ordering!()