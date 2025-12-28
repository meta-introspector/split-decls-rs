macro_rules! deps {
    () => {
        ForEachCb!();
    };
}

macro_rules! foreach_c {
    () => {
        deps!();
        extern "C" fn foreach_c (buf : * const c_void , size : size_t , data : * mut c_void) -> c_int { unsafe { let buf = slice :: from_raw_parts (buf as * const u8 , size as usize) ; let r = panic :: wrap (| | { let data = data as * mut & mut ForEachCb < '_ > ; (* data) (buf) }) ; if r == Some (true) { 0 } else { - 1 } } }
    };
}

foreach_c!();