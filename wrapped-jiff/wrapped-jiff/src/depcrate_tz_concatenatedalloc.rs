// Generated macro for alloc (function)
macro_rules! Depcrate_tz_concatenatedalloc {
() => {
// Module: crate::tz::concatenated
// Provides: {"alloc"}
// Dependencies: {}
# [doc = " Allocates `additional` extra bytes on the `Vec` given and set them to `0`."] # [doc = ""] # [doc = " This specifically will never do an \"OOM panic\" and will instead return an"] # [doc = " error (courtesy of `Vec::try_reserve_exact`). It will also return an error"] # [doc = " without even trying the allocation if it's deemed to be \"too big.\""] # [doc = ""] # [doc = " This is used so that we are extra careful about creating allocations based"] # [doc = " on integers parsed from concatenated TZif data. Generally speaking, the"] # [doc = " data we parse should be \"trusted\" (since it's probably not writable by"] # [doc = " anyone other than `root`), but who knows where this code will ultimately be"] # [doc = " used. So we try pretty hard to avoid panicking (even for OOM)."] # [doc = ""] # [doc = " To be clear, we probably could panic on the error path. The goal here"] # [doc = " isn't to avoid OOM because you can't allocate 10 bytes---Jiff isn't robust"] # [doc = " enough in that kind of environment by far. The goal is to avoid OOM for"] # [doc = " exorbitantly large allocations through some kind of attack vector."] fn alloc (bytes : & mut Vec < u8 > , additional : usize) -> Result < () , Error > { const LIMIT : usize = 10 * 1 << 20 ; if additional > LIMIT { return Err (err ! ("attempted to allocate more than {LIMIT} bytes \
             while reading concatenated TZif data, which \
             exceeds a heuristic limit to prevent huge allocations \
             (please file a bug if this error is inappropriate)" ,)) ; } bytes . try_reserve_exact (additional) . map_err (| _ | { err ! ("failed to allocation {additional} bytes \
             for reading concatenated TZif data") }) ? ; let new_len = bytes . len () . checked_add (additional) . ok_or_else (| | err ! ("total allocation length overflowed `usize`")) ? ; bytes . resize (new_len , 0) ; Ok (()) }
};
}
