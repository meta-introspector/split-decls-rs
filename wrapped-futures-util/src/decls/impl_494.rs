macro_rules! impl_494 {
    () => {
        impl < St : Stream , S , Fut , F > Scan < St , S , Fut , F > { # [doc = " Checks if internal state is `None`."] fn is_done_taking (& self) -> bool { self . state . is_empty () } }
    };
}

impl_494!()