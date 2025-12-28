macro_rules! utils {
    () => {
        pub (crate) mod utils { pub enum Read { Known (gix_features :: io :: pipe :: Reader) , Unknown (Box < dyn std :: io :: Read >) , } impl std :: io :: Read for Read { fn read (& mut self , buf : & mut [u8]) -> std :: io :: Result < usize > { match self { Read :: Known (r) => r . read (buf) , Read :: Unknown (r) => r . read (buf) , } } } }
    };
}

utils!();