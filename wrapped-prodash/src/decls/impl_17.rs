macro_rules! deps {
    () => {
        Key!();
        MessageRingBuffer!();
        Item!();
        Options!();
        Root!();
        HashMap!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl From < Options > for Root { fn from (Options { initial_capacity , message_buffer_capacity , } : Options ,) -> Self { Root { inner : Mutex :: new (Item { highest_child_id : 0 , value : Arc :: new (AtomicUsize :: default ()) , key : Key :: default () , tree : Arc :: new (crate :: tree :: HashMap :: with_capacity (initial_capacity)) , messages : Arc :: new (Mutex :: new (MessageRingBuffer :: with_capacity (message_buffer_capacity))) , }) , } } }
    };
}

impl_17!()