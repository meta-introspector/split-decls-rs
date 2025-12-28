macro_rules! deps {
    () => {
        Handler!();
        Inner!();
    };
}

macro_rules! seek_cb {
    () => {
        deps!();
        extern "C" fn seek_cb < H : Handler > (data : * mut c_void , offset : curl_sys :: curl_off_t , origin : c_int ,) -> c_int { panic :: catch (| | unsafe { let from = if origin == libc :: SEEK_SET { SeekFrom :: Start (offset as u64) } else { panic ! ("unknown origin from libcurl: {}" , origin) ; } ; (* (data as * mut Inner < H >)) . handler . seek (from) as c_int }) . unwrap_or (! 0) }
    };
}

seek_cb!()