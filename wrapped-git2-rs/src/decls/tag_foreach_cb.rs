macro_rules! deps {
    () => {
        TagForeachData!();
        Binding!();
        Oid!();
    };
}

macro_rules! tag_foreach_cb {
    () => {
        deps!();
        # [doc = " c callback forwarding to rust callback inside `TagForeachData`"] # [doc = " see original: <https://libgit2.org/libgit2/#HEAD/group/callback/git_tag_foreach_cb>"] pub (crate) extern "C" fn tag_foreach_cb (name : * const c_char , oid : * mut git_oid , payload : * mut c_void ,) -> c_int { panic :: wrap (| | unsafe { let id : Oid = Binding :: from_raw (oid as * const _) ; let name = CStr :: from_ptr (name) ; let name = name . to_bytes () ; let payload = & mut * (payload as * mut TagForeachData < '_ >) ; let cb = & mut payload . cb ; let res = cb (id , name) ; if res { 0 } else { - 1 } }) . unwrap_or (- 1) }
    };
}

tag_foreach_cb!();