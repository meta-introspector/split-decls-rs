macro_rules! deps {
    () => {
        Kind!();
    };
}

macro_rules! Header {
    () => {
        deps!();
        # [doc = " Information about an object, which includes its kind and the amount of bytes it would have when obtained."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone , Copy)] pub struct Header { # [doc = " The kind of object."] pub kind : Kind , # [doc = " The object's size in bytes, or the size of the buffer when it's retrieved in full."] pub size : u64 , }
    };
}

Header!()