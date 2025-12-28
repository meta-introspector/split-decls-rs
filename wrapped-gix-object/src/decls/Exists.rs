macro_rules! Exists {
    () => {
        # [doc = " Check if an object is present in an object store."] pub trait Exists { # [doc = " Returns `true` if the object exists in the database."] fn exists (& self , id : & gix_hash :: oid) -> bool ; }
    };
}

Exists!()