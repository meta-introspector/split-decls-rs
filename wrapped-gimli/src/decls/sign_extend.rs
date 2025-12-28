macro_rules! sign_extend {
    () => {
        # [doc = " Convert a u64 to an i64, with sign extension if required."] # [doc = ""] # [doc = " This is primarily used when needing to treat `Value::Generic`"] # [doc = " as a signed value."] # [inline] fn sign_extend (value : u64 , mask : u64) -> i64 { let value = (value & mask) as i64 ; let sign = ((mask >> 1) + 1) as i64 ; (value ^ sign) . wrapping_sub (sign) }
    };
}

sign_extend!();