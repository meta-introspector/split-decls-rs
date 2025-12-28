macro_rules! deps {
    () => {
        DropCounter!();
        Element!();
    };
}

macro_rules! impl_370 {
    () => {
        deps!();
        impl DropCounter { fn created (& self) -> usize { self . created . load (Ordering :: SeqCst) } fn dropped (& self) -> usize { self . dropped . load (Ordering :: SeqCst) } fn element (& self) -> Element < '_ > { self . created . fetch_add (1 , Ordering :: SeqCst) ; Element (& self . dropped) } fn assert_drop_count (& self) { assert_eq ! (self . created () , self . dropped () , "Expected {} dropped elements, but found {}" , self . created () , self . dropped ()) ; } }
    };
}

impl_370!()