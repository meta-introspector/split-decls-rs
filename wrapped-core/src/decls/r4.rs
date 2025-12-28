macro_rules! r4 {
    () => {
        const fn r4 (mut block : [u32 ; 16] , v : u32 , mut w : u32 , x : u32 , y : u32 , mut z : u32 , i : usize ,) -> ([u32 ; 16] , u32 , u32) { block [i] = blk (& block , i) ; let n = (w ^ x ^ y) . wrapping_add (block [i]) . wrapping_add (0xca62_c1d6) . wrapping_add (rol (v , 5)) ; z = z . wrapping_add (n) ; w = rol (w , 30) ; (block , w , z) }
    };
}

r4!()