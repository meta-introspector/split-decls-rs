macro_rules! deps {
    () => {
        TestIdGuard!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl Drop for TestIdGuard { fn drop (& mut self) { TEST_ID . with (| n | * n . borrow_mut () = None) ; } }
    };
}

impl_77!();