macro_rules! deps {
    () => {
        Serializer!();
        Result!();
    };
}

macro_rules! to_writer {
    () => {
        deps!();
        # [doc = " Serialize the given data structure as JSON into the I/O stream."] # [doc = ""] # [doc = " Serialization guarantees it only feeds valid UTF-8 sequences to the writer."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " Serialization can fail if `T`'s implementation of `Serialize` decides to"] # [doc = " fail, or if `T` contains a map with non-string keys."] # [inline] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] pub fn to_writer < W , T > (writer : W , value : & T) -> Result < () > where W : io :: Write , T : ? Sized + Serialize , { let mut ser = Serializer :: new (writer) ; value . serialize (& mut ser) }
    };
}

to_writer!();