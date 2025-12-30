// Generated macro for hashv (function)
macro_rules! Depcratehashv {
() => {
// Module: crate
// Provides: {"hashv"}
// Dependencies: {}
# [doc = " Return a Blake3 hash for the given data."] # [cfg_attr (any (target_os = "solana" , target_arch = "bpf") , inline (always))] pub fn hashv (vals : & [& [u8]]) -> Hash { # [cfg (not (any (target_os = "solana" , target_arch = "bpf")))] { # [cfg (feature = "blake3")] { let mut hasher = Hasher :: default () ; hasher . hashv (vals) ; hasher . result () } # [cfg (not (feature = "blake3"))] { core :: hint :: black_box (vals) ; panic ! ("hashv is only available on target `solana` or with the `blake3` feature enabled on this crate") } } # [cfg (any (target_os = "solana" , target_arch = "bpf"))] { let mut hash_result = core :: mem :: MaybeUninit :: < [u8 ; solana_hash :: HASH_BYTES] > :: uninit () ; unsafe { solana_define_syscall :: definitions :: sol_blake3 (vals as * const _ as * const u8 , vals . len () as u64 , hash_result . as_mut_ptr () as * mut u8 ,) ; Hash :: new_from_array (hash_result . assume_init ()) } } }
};
}
