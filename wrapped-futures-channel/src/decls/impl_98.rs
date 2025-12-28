macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl State { fn is_closed (& self) -> bool { ! self . is_open && self . num_messages == 0 } fn size_hint (& self) -> (usize , Option < usize >) { if self . is_open { (self . num_messages , None) } else { (self . num_messages , Some (self . num_messages)) } } }
    };
}

impl_98!();