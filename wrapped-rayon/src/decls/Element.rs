macro_rules! Element {
    () => {
        struct Element < 'a > (& 'a AtomicUsize) ;
    };
}

Element!();