macro_rules! LIBZ_RS_SYS_VERSION {
    () => {
        const LIBZ_RS_SYS_VERSION : & str = concat ! (libz_rs_sys_version ! () , "\0") ;
    };
}

LIBZ_RS_SYS_VERSION!();