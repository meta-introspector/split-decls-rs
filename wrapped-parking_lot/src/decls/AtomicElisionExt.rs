macro_rules! AtomicElisionExt {
    () => {
        pub trait AtomicElisionExt { type IntType ; fn elision_compare_exchange_acquire (& self , current : Self :: IntType , new : Self :: IntType ,) -> Result < Self :: IntType , Self :: IntType > ; fn elision_fetch_sub_release (& self , val : Self :: IntType) -> Self :: IntType ; }
    };
}

AtomicElisionExt!()