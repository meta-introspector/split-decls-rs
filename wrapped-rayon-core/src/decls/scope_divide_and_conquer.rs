macro_rules! scope_divide_and_conquer {
    () => {
        # [test] fn scope_divide_and_conquer () { let counter_p = & AtomicUsize :: new (0) ; scope (| s | s . spawn (move | s | divide_and_conquer (s , counter_p , 1024))) ; let counter_s = & AtomicUsize :: new (0) ; divide_and_conquer_seq (counter_s , 1024) ; let p = counter_p . load (Ordering :: SeqCst) ; let s = counter_s . load (Ordering :: SeqCst) ; assert_eq ! (p , s) ; }
    };
}

scope_divide_and_conquer!()