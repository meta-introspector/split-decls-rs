macro_rules! macro_13 {
    () => {
        bitflags :: bitflags ! { # [doc = " Whether something can be read or written."] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] # [derive (Debug)] pub struct ReadWrite : u8 { # [doc = " The item can be read."] const READ = 1 << 0 ; # [doc = " The item can be written"] const WRITE = 1 << 1 ; } }
    };
}

macro_13!()