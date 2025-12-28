macro_rules! deps {
    () => {
        SharedTempFile!();
    };
}

macro_rules! LockWriter {
    () => {
        deps!();
        pub (crate) struct LockWriter { pub writer : SharedTempFile , }
    };
}

LockWriter!()