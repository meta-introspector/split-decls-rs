macro_rules! deps {
    () => {
        GUID!();
        ConstBuffer!();
    };
}

macro_rules! impl_132 {
    () => {
        deps!();
        impl GUID { # [doc = " Creates a unique `GUID` value."] pub fn new () -> Result < Self > { let mut guid = Self :: zeroed () ; let result = unsafe { imp :: UuidCreate (& mut guid as * mut _ as _) } ; if matches ! (result , 0 | imp :: RPC_S_UUID_LOCAL_ONLY) { Ok (guid) } else { Err (Error :: from_hresult (WIN32_ERROR (result as u32) . to_hresult ())) } } # [doc = " Creates a `GUID` represented by the all-zero byte-pattern."] pub const fn zeroed () -> Self { Self { data1 : 0 , data2 : 0 , data3 : 0 , data4 : [0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , } } # [doc = " Creates a `GUID` with the given constant values."] pub const fn from_values (data1 : u32 , data2 : u16 , data3 : u16 , data4 : [u8 ; 8]) -> Self { Self { data1 , data2 , data3 , data4 , } } # [doc = " Creates a `GUID` from a `u128` value."] pub const fn from_u128 (uuid : u128) -> Self { Self { data1 : (uuid >> 96) as u32 , data2 : ((uuid >> 80) & 0xffff) as u16 , data3 : ((uuid >> 64) & 0xffff) as u16 , data4 : (uuid as u64) . to_be_bytes () , } } # [doc = " Converts a `GUID` to a `u128` value."] pub const fn to_u128 (& self) -> u128 { ((self . data1 as u128) << 96) + ((self . data2 as u128) << 80) + ((self . data3 as u128) << 64) + u64 :: from_be_bytes (self . data4) as u128 } # [doc = " Creates a `GUID` for a \"generic\" WinRT type."] pub const fn from_signature (signature : imp :: ConstBuffer) -> Self { let data = imp :: ConstBuffer :: from_slice (& [0x11 , 0xf4 , 0x7a , 0xd5 , 0x7b , 0x73 , 0x42 , 0xc0 , 0xab , 0xae , 0x87 , 0x8b , 0x1e , 0x16 , 0xad , 0xee ,]) ; let data = data . push_other (signature) ; let bytes = imp :: sha1 (& data) . bytes () ; let first = u32 :: from_be_bytes ([bytes [0] , bytes [1] , bytes [2] , bytes [3]]) ; let second = u16 :: from_be_bytes ([bytes [4] , bytes [5]]) ; let mut third = u16 :: from_be_bytes ([bytes [6] , bytes [7]]) ; third = (third & 0x0fff) | (5 << 12) ; let fourth = (bytes [8] & 0x3f) | 0x80 ; Self :: from_values (first , second , third , [fourth , bytes [9] , bytes [10] , bytes [11] , bytes [12] , bytes [13] , bytes [14] , bytes [15] ,] ,) } }
    };
}

impl_132!()