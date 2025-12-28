macro_rules! deps {
    () => {
        RuntimeName!();
        Interface!();
        RuntimeType!();
        ConstBuffer!();
        GUID!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        impl ConstBuffer { pub const fn for_class < C : crate :: RuntimeName , I : crate :: RuntimeType > () -> Self { Self :: new () . push_slice (b"rc(") . push_slice (C :: NAME . as_bytes ()) . push (b';') . push_other (I :: SIGNATURE) . push (b')') } pub const fn for_interface < T : crate :: Interface > () -> Self { Self :: new () . push_guid (& T :: IID) } pub const fn from_slice (slice : & [u8]) -> Self { let s = Self :: new () ; s . push_slice (slice) } pub const fn new () -> Self { Self { data : [0 ; BUFFER_SIZE] , head : 0 , } } pub const fn push_slice (self , slice : & [u8]) -> Self { self . push_amount (slice , slice . len ()) } const fn get (& self , index : usize) -> u8 { self . data [index] } const fn len (& self) -> usize { self . head } pub fn as_slice (& self) -> & [u8] { & self . data [.. self . head] } pub const fn push_other (self , other : Self) -> Self { self . push_amount (& other . data , other . len ()) } const fn push (mut self , value : u8) -> Self { self . data [self . head] = value ; self . head += 1 ; self } const fn push_hex_u8 (self , value : u8) -> Self { const fn digit (mut value : u8) -> u8 { value &= 0xF ; if value < 10 { b'0' + value } else { b'a' + (value - 10) } } self . push (digit (value >> 4)) . push (digit (value)) } const fn push_hex_u16 (self , value : u16) -> Self { self . push_hex_u8 ((value >> 8) as u8) . push_hex_u8 ((value & 0xFF) as u8) } const fn push_hex_u32 (self , value : u32) -> Self { self . push_hex_u16 ((value >> 16) as u16) . push_hex_u16 ((value & 0xFFFF) as u16) } const fn push_amount (mut self , slice : & [u8] , amount : usize) -> Self { let mut i = 0 ; while i < amount { self . data [self . head + i] = slice [i] ; i += 1 ; } self . head += i ; self } const fn push_guid (self , guid : & crate :: GUID) -> Self { self . push (b'{') . push_hex_u32 (guid . data1) . push (b'-') . push_hex_u16 (guid . data2) . push (b'-') . push_hex_u16 (guid . data3) . push (b'-') . push_hex_u16 (((guid . data4 [0] as u16) << 8) | guid . data4 [1] as u16) . push (b'-') . push_hex_u16 (((guid . data4 [2] as u16) << 8) | guid . data4 [3] as u16) . push_hex_u16 (((guid . data4 [4] as u16) << 8) | guid . data4 [5] as u16) . push_hex_u16 (((guid . data4 [6] as u16) << 8) | guid . data4 [7] as u16) . push (b'}') } }
    };
}

impl_78!();