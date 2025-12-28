macro_rules! deps {
    () => {
        Kind!();
    };
}

macro_rules! Object {
    () => {
        deps!();
        # [doc = " A way of storing and retrieving entire objects to and from a cache."] pub trait Object { # [doc = " Put the object going by `id` of `kind` with `data` into the cache."] fn put (& mut self , id : gix_hash :: ObjectId , kind : gix_object :: Kind , data : & [u8]) ; # [doc = " Try to retrieve the object named `id` and place its data into `out` if available and return `Some(kind)` if found."] fn get (& mut self , id : & gix_hash :: ObjectId , out : & mut Vec < u8 >) -> Option < gix_object :: Kind > ; }
    };
}

Object!();