macro_rules! Blobs {
    () => {
        pub struct Blobs { map : HashMap < Vec < u8 > , id :: BlobId > , stream : Vec < u8 > , }
    };
}

Blobs!();