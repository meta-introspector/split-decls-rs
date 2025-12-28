macro_rules! Sha256 {
    () => {
        pub struct Sha256 (Sha2_sha256) ;
    };
}

Sha256!();