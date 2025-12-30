// Generated macro for hash (module)
macro_rules! Depcratehash {
() => {
// Module: crate
// Provides: {"hash"}
// Dependencies: {}
# [doc = ""] pub mod hash { # [doc = " A Hasher for usage with `HashMap` keys that are already robust hashes (like an `ObjectId`)."] # [doc = " The first `8` bytes of the hash are used as the `HashMap` hash"] # [derive (Default , Clone , Copy)] pub struct Hasher (u64) ; macro_rules ! panic_other_writers { ($ func : ident , $ type : ty) => { # [cold] fn $ func (& mut self , _i : $ type) { panic ! ("This hasher only supports manually verified `Hash` implementations") } } ; } impl std :: hash :: Hasher for Hasher { fn finish (& self) -> u64 { self . 0 } # [inline (always)] fn write (& mut self , bytes : & [u8]) { self . 0 = u64 :: from_ne_bytes (bytes [.. 8] . try_into () . unwrap ()) ; } panic_other_writers ! (write_u8 , u8) ; panic_other_writers ! (write_u16 , u16) ; panic_other_writers ! (write_u32 , u32) ; panic_other_writers ! (write_u64 , u64) ; panic_other_writers ! (write_u128 , u128) ; panic_other_writers ! (write_usize , usize) ; panic_other_writers ! (write_i8 , i8) ; panic_other_writers ! (write_i16 , i16) ; panic_other_writers ! (write_i32 , i32) ; panic_other_writers ! (write_i64 , i64) ; panic_other_writers ! (write_i128 , i128) ; panic_other_writers ! (write_isize , isize) ; } # [doc = " A Hasher for usage with `HashMap` keys that are already robust hashes (like an `ObjectId`)."] # [doc = " The first `8` bytes of the hash are used as the `HashMap` hash"] # [derive (Default , Clone , Copy)] pub struct Builder ; impl std :: hash :: BuildHasher for Builder { type Hasher = Hasher ; fn build_hasher (& self) -> Self :: Hasher { Hasher :: default () } } }
};
}
