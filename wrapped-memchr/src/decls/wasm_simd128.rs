macro_rules! deps {
    () => {
        SensibleMoveMask!();
        Vector!();
        Mask!();
    };
}

macro_rules! wasm_simd128 {
    () => {
        deps!();
        # [cfg (all (target_arch = "wasm32" , target_feature = "simd128"))] mod wasm_simd128 { use core :: arch :: wasm32 :: * ; use super :: { SensibleMoveMask , Vector } ; impl Vector for v128 { const BYTES : usize = 16 ; const ALIGN : usize = Self :: BYTES - 1 ; type Mask = SensibleMoveMask ; # [inline (always)] unsafe fn splat (byte : u8) -> v128 { u8x16_splat (byte) } # [inline (always)] unsafe fn load_aligned (data : * const u8) -> v128 { * data . cast () } # [inline (always)] unsafe fn load_unaligned (data : * const u8) -> v128 { v128_load (data . cast ()) } # [inline (always)] unsafe fn movemask (self) -> SensibleMoveMask { SensibleMoveMask (u8x16_bitmask (self) . into ()) } # [inline (always)] unsafe fn cmpeq (self , vector2 : Self) -> v128 { u8x16_eq (self , vector2) } # [inline (always)] unsafe fn and (self , vector2 : Self) -> v128 { v128_and (self , vector2) } # [inline (always)] unsafe fn or (self , vector2 : Self) -> v128 { v128_or (self , vector2) } } }
    };
}

wasm_simd128!();