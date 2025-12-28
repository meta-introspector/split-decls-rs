macro_rules! deps {
    () => {
        Sealed!();
        SerializedBuffer!();
        BufferKind!();
        BlockBuffer!();
        Error!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl < BS : ArraySize , K : BufferKind > BlockBuffer < BS , K > where BS : core :: ops :: Add < K :: Overhead > , Sum < BS , K :: Overhead > : ArraySize , { # [doc = " Serialize buffer into a byte array."] pub fn serialize (& self) -> SerializedBuffer < BS , K > { let mut buf = SerializedBuffer :: < BS , K > :: default () ; let data = self . get_data () ; let (pos , block) = buf . split_at_mut (1) ; pos [0] = u8 :: try_from (data . len ()) . expect ("buffer size is smaller than 256") ; block [.. data . len ()] . copy_from_slice (data) ; buf } # [doc = " Deserialize buffer from a byte array."] pub fn deserialize (buf : & SerializedBuffer < BS , K >) -> Result < Self , Error > { let (pos , block) = buf . split_at (1) ; let pos = usize :: from (pos [0]) ; if ! < K as sealed :: Sealed > :: invariant (pos , BS :: USIZE) { return Err (Error) ; } let (data , tail) = block . split_at (pos) ; if tail . iter () . any (| & b | b != 0) { return Err (Error) ; } let mut res = Self :: default () ; unsafe { res . set_data_unchecked (data) } ; Ok (res) } }
    };
}

impl_30!()