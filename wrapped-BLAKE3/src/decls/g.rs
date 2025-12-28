macro_rules! g {
    () => {
        # [inline (always)] fn g (state : & mut [u32 ; 16] , a : usize , b : usize , c : usize , d : usize , x : u32 , y : u32) { state [a] = state [a] . wrapping_add (state [b]) . wrapping_add (x) ; state [d] = (state [d] ^ state [a]) . rotate_right (16) ; state [c] = state [c] . wrapping_add (state [d]) ; state [b] = (state [b] ^ state [c]) . rotate_right (12) ; state [a] = state [a] . wrapping_add (state [b]) . wrapping_add (y) ; state [d] = (state [d] ^ state [a]) . rotate_right (8) ; state [c] = state [c] . wrapping_add (state [d]) ; state [b] = (state [b] ^ state [c]) . rotate_right (7) ; }
    };
}

g!();