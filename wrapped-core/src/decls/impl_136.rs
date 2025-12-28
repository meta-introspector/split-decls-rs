macro_rules! deps {
    () => {
        GUID!();
    };
}

macro_rules! impl_136 {
    () => {
        deps!();
        impl TryFrom < & str > for GUID { type Error = Error ; fn try_from (from : & str) -> Result < Self > { if from . len () != 36 { return Err (invalid_guid ()) ; } let bytes = & mut from . bytes () ; let mut guid = Self :: zeroed () ; guid . data1 = try_u32 (bytes , true) ? ; guid . data2 = try_u16 (bytes , true) ? ; guid . data3 = try_u16 (bytes , true) ? ; guid . data4 [0] = try_u8 (bytes , false) ? ; guid . data4 [1] = try_u8 (bytes , true) ? ; guid . data4 [2] = try_u8 (bytes , false) ? ; guid . data4 [3] = try_u8 (bytes , false) ? ; guid . data4 [4] = try_u8 (bytes , false) ? ; guid . data4 [5] = try_u8 (bytes , false) ? ; guid . data4 [6] = try_u8 (bytes , false) ? ; guid . data4 [7] = try_u8 (bytes , false) ? ; Ok (guid) } }
    };
}

impl_136!()