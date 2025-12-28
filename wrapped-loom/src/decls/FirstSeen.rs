macro_rules! FirstSeen {
    () => {
        # [derive (Debug)] struct FirstSeen ([u16 ; MAX_THREADS]) ;
    };
}

FirstSeen!();