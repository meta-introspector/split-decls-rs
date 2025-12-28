macro_rules! deps {
    () => {
        AtomicCompareExchange!();
    };
}

macro_rules! delegate_cas {
    () => {
        deps!();
        # [allow (unused_macros)] macro_rules ! delegate_cas { ($ ty : ident , $ base : ident) => { const _ : () = { assert ! (mem :: size_of ::<$ ty > () == mem :: size_of ::<$ base > ()) ; assert ! (mem :: align_of ::<$ ty > () == mem :: align_of ::<$ base > ()) ; } ; impl AtomicCompareExchange for $ ty { # [inline] unsafe fn atomic_compare_exchange (dst : * mut MaybeUninit < Self >, current : MaybeUninit < Self >, new : MaybeUninit < Self >, success : Ordering , failure : Ordering ,) -> (MaybeUninit < Self >, bool) { unsafe { let (out , ok) = <$ base as AtomicCompareExchange >:: atomic_compare_exchange (dst . cast ::< MaybeUninit <$ base >> () , mem :: transmute ::< MaybeUninit < Self >, MaybeUninit <$ base >> (current) , mem :: transmute ::< MaybeUninit < Self >, MaybeUninit <$ base >> (new) , success , failure ,) ; (mem :: transmute ::< MaybeUninit <$ base >, MaybeUninit < Self >> (out) , ok) } } # [inline] unsafe fn atomic_compare_exchange_weak (dst : * mut MaybeUninit < Self >, current : MaybeUninit < Self >, new : MaybeUninit < Self >, success : Ordering , failure : Ordering ,) -> (MaybeUninit < Self >, bool) { unsafe { let (out , ok) = <$ base as AtomicCompareExchange >:: atomic_compare_exchange_weak (dst . cast ::< MaybeUninit <$ base >> () , mem :: transmute ::< MaybeUninit < Self >, MaybeUninit <$ base >> (current) , mem :: transmute ::< MaybeUninit < Self >, MaybeUninit <$ base >> (new) , success , failure ,) ; (mem :: transmute ::< MaybeUninit <$ base >, MaybeUninit < Self >> (out) , ok) } } } } ; }
    };
}

delegate_cas!()