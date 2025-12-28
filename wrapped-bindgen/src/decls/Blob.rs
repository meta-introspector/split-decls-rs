macro_rules! deps {
    () => {
        File!();
    };
}

macro_rules! Blob {
    () => {
        deps!();
        pub struct Blob { file : & 'static File , slice : & 'static [u8] , }
    };
}

Blob!();