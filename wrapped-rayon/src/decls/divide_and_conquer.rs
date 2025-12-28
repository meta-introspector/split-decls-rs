macro_rules! divide_and_conquer {
    () => {
        fn divide_and_conquer < 'scope > (scope : & Scope < 'scope > , counter : & 'scope AtomicUsize , size : usize) { if size > 1 { scope . spawn (move | scope | divide_and_conquer (scope , counter , size / 2)) ; scope . spawn (move | scope | divide_and_conquer (scope , counter , size / 2)) ; } else { counter . fetch_add (1 , Ordering :: SeqCst) ; } }
    };
}

divide_and_conquer!();