macro_rules! block_mix {
    () => {
        # [rustfmt :: skip] fn block_mix (tmp : & mut [u32] , inn : & [u32] , out : & mut [u32] , r : usize) { block_copy (tmp , & inn [(2 * r - 1) * 16 ..] , 16) ; for i in (0 .. 2 * r) . step_by (2) { salsa_xor (tmp , & inn [i * 16 ..] , & mut out [i * 8 ..]) ; salsa_xor (tmp , & inn [i * 16 + 16 ..] , & mut out [i * 8 + r * 16 ..]) ; } }
    };
}

block_mix!()