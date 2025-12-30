// Generated macro for _ (const)
macro_rules! Depcrate_ {
() => {
// Module: crate
// Provides: {"_"}
// Dependencies: {}
# [cfg (all (feature = "memchr" , not (feature = "raw_os_str")))] const _ : & str = env ! ("__OS_STR_BYTES_CI" , concat ! ("The 'memchr' feature is useless when 'raw_os_str' is disabled; it \
         should be disabled too." ,) ,) ;
};
}
