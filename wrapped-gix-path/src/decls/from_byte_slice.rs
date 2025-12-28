macro_rules! from_byte_slice {
    () => {
        # [doc = " Similar to [`try_from_byte_slice()`], but will **panic** if there is ill-formed UTF-8 in the `input`."] pub fn from_byte_slice (input : & [u8]) -> & Path { try_from_byte_slice (input) . expect ("well-formed UTF-8 on windows") }
    };
}

from_byte_slice!();