macro_rules! deps {
    () => {
        Store!();
    };
}

macro_rules! impl_120 {
    () => {
        deps!();
        # [doc = " Access"] impl Store { # [doc = " Return the path to the object with `id`."] # [doc = ""] # [doc = " Note that is may not exist yet."] pub fn object_path (& self , id : & gix_hash :: oid) -> PathBuf { loose :: hash_path (id , self . path . clone ()) } }
    };
}

impl_120!();