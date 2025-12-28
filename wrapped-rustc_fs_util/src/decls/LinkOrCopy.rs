macro_rules! LinkOrCopy {
    () => {
        pub enum LinkOrCopy { Link , Copy , }
    };
}

LinkOrCopy!()