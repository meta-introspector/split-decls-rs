macro_rules! Mode {
    () => {
        # [doc = " Various ways in which a pack and index can be verified"] # [derive (Default , Debug , Eq , PartialEq , Hash , Clone , Copy)] pub enum Mode { # [doc = " Validate the object hash and CRC32"] HashCrc32 , # [doc = " Validate hash and CRC32, and decode each non-Blob object."] # [doc = " Each object should be valid, i.e. be decodable."] HashCrc32Decode , # [doc = " Validate hash and CRC32, and decode and encode each non-Blob object."] # [doc = " Each object should yield exactly the same hash when re-encoded."] # [default] HashCrc32DecodeEncode , }
    };
}

Mode!();