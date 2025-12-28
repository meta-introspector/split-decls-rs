macro_rules! deps {
    () => {
        AtomicLoad!();
        AtomicStore!();
    };
}

macro_rules! delegate_load_store {
    () => {
        deps!();
        # [allow (unused_macros)] macro_rules ! delegate_load_store { ($ ty : ident , $ base : ident) => { const _ : () = { assert ! (mem :: size_of ::<$ ty > () == mem :: size_of ::<$ base > ()) ; assert ! (mem :: align_of ::<$ ty > () == mem :: align_of ::<$ base > ()) ; } ; impl AtomicLoad for $ ty { # [inline] unsafe fn atomic_load (src : * const MaybeUninit < Self >, order : Ordering ,) -> MaybeUninit < Self > { unsafe { mem :: transmute ::< MaybeUninit <$ base >, MaybeUninit < Self >> (<$ base as AtomicLoad >:: atomic_load (src . cast ::< MaybeUninit <$ base >> () , order) ,) } } } impl AtomicStore for $ ty { # [inline] unsafe fn atomic_store (dst : * mut MaybeUninit < Self >, val : MaybeUninit < Self >, order : Ordering ,) { unsafe { <$ base as AtomicStore >:: atomic_store (dst . cast ::< MaybeUninit <$ base >> () , mem :: transmute ::< MaybeUninit < Self >, MaybeUninit <$ base >> (val) , order ,) ; } } } } ; }
    };
}

delegate_load_store!()