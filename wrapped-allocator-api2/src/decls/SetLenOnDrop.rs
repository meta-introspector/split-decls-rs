macro_rules! SetLenOnDrop {
    () => {
        pub (super) struct SetLenOnDrop < 'a > { len : & 'a mut usize , local_len : usize , }
    };
}

SetLenOnDrop!()