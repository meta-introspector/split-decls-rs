macro_rules! from_c_string {
    () => {
        pub fn from_c_string < 'a > (ptr : * const c_char) -> & 'a str { unsafe { CStr :: from_ptr (ptr) } . to_str () . expect ("CStr::to_str failed") }
    };
}

from_c_string!()