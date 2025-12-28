macro_rules! wake_panic {
    () => {
        unsafe fn wake_panic (_data : * const ()) { if ! std :: thread :: panicking () { panic ! ("should not be woken") ; } }
    };
}

wake_panic!()