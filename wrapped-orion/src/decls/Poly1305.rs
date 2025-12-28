macro_rules! Poly1305 {
    () => {
        # [derive (Clone)] # [doc = " Poly1305 streaming state."] pub struct Poly1305 { a : fiat_poly1305_tight_field_element , r : fiat_poly1305_loose_field_element , s : [u32 ; 4] , leftover : usize , buffer : [u8 ; POLY1305_BLOCKSIZE] , is_finalized : bool , }
    };
}

Poly1305!()