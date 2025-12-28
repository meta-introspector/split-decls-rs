macro_rules! deps {
    () => {
        RequestWriter!();
        ExtendedBufRead!();
    };
}

macro_rules! MessageKind {
    () => {
        deps!();
        # [doc = " The kind of packet line to write when transforming a `RequestWriter` into an `ExtendedBufRead`."] # [doc = ""] # [doc = " Both the type and the trait have different implementations for blocking vs async I/O."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone , Copy)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub enum MessageKind { # [doc = " A `flush` packet."] Flush , # [doc = " A V2 delimiter."] Delimiter , # [doc = " The end of a response."] ResponseEnd , # [doc = " The given text."] Text (& 'static [u8]) , }
    };
}

MessageKind!();