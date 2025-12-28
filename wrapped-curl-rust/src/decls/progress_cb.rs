macro_rules! deps {
    () => {
        Inner!();
        Handler!();
    };
}

macro_rules! progress_cb {
    () => {
        deps!();
        extern "C" fn progress_cb < H : Handler > (data : * mut c_void , dltotal : c_double , dlnow : c_double , ultotal : c_double , ulnow : c_double ,) -> c_int { let keep_going = panic :: catch (| | unsafe { (* (data as * mut Inner < H >)) . handler . progress (dltotal , dlnow , ultotal , ulnow) }) . unwrap_or (false) ; if keep_going { 0 } else { 1 } }
    };
}

progress_cb!()