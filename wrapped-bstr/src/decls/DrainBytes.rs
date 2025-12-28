macro_rules! DrainBytes {
    () => {
        # [doc = " A draining byte oriented iterator for `Vec<u8>`."] # [doc = ""] # [doc = " This iterator is created by"] # [doc = " [`ByteVec::drain_bytes`](trait.ByteVec.html#method.drain_bytes)."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Basic usage:"] # [doc = ""] # [doc = " ```"] # [doc = " use bstr::ByteVec;"] # [doc = ""] # [doc = " let mut s = Vec::from(\"foobar\");"] # [doc = " {"] # [doc = "     let mut drainer = s.drain_bytes(2..4);"] # [doc = "     assert_eq!(drainer.next(), Some(b'o'));"] # [doc = "     assert_eq!(drainer.next(), Some(b'b'));"] # [doc = "     assert_eq!(drainer.next(), None);"] # [doc = " }"] # [doc = " assert_eq!(s, \"foar\".as_bytes());"] # [doc = " ```"] # [derive (Debug)] pub struct DrainBytes < 'a > { it : vec :: Drain < 'a , u8 > , }
    };
}

DrainBytes!();