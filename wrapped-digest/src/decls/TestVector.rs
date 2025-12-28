macro_rules! TestVector {
    () => {
        # [doc = " Test vector for hash functions"] # [derive (Debug , Clone , Copy)] pub struct TestVector { # [doc = " Input data"] pub input : & 'static [u8] , # [doc = " Output hash"] pub output : & 'static [u8] , }
    };
}

TestVector!()