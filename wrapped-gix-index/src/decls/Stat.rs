macro_rules! deps {
    () => {
        Time!();
    };
}

macro_rules! Stat {
    () => {
        deps!();
        # [doc = " An entry's filesystem stat information."] # [derive (Debug , Default , PartialEq , Eq , Hash , Ord , PartialOrd , Clone , Copy)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct Stat { # [doc = " Modification time"] pub mtime : stat :: Time , # [doc = " Creation time"] pub ctime : stat :: Time , # [doc = " Device number"] pub dev : u32 , # [doc = " Inode number"] pub ino : u32 , # [doc = " User id of the owner"] pub uid : u32 , # [doc = " Group id of the owning group"] pub gid : u32 , # [doc = " The size of bytes on disk. Capped to u32 so files bigger than that will need thorough additional checking"] pub size : u32 , }
    };
}

Stat!()