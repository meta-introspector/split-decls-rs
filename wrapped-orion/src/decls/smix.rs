macro_rules! smix {
    () => {
        # [allow (non_snake_case)] # [allow (clippy :: needless_range_loop)] fn smix (b : & mut [u8] , r : usize , N : usize , v : & mut [u32] , x : & mut [u32] , y : & mut [u32]) { let mut tmp = [0u32 ; 16] ; let R = 32 * r ; let mut j = 0 ; for i in 0 .. R { x [i] = u32 :: from_le_bytes (b [j .. j + 4] . try_into () . unwrap ()) ; j += 4 ; } for i in (0 .. N) . step_by (2) { block_copy (& mut v [i * R ..] , x , R) ; block_mix (& mut tmp , x , y , r) ; block_copy (& mut v [(i + 1) * R ..] , y , R) ; block_mix (& mut tmp , y , x , r) ; } for _ in (0 .. N) . step_by (2) { let j = (integer (x , r) & (N - 1) as u64) as usize ; block_xor (x , & v [j * R ..] , R) ; block_mix (& mut tmp , x , y , r) ; let j = (integer (y , r) & (N - 1) as u64) as usize ; block_xor (y , & v [j * R ..] , R) ; block_mix (& mut tmp , y , x , r) ; } let mut j = 0 ; for v in & x [.. R] { b [j .. j + 4] . copy_from_slice (& v . to_le_bytes ()) ; j += 4 ; } }
    };
}

smix!()