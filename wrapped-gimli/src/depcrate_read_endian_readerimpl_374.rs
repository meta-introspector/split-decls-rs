// Generated macro for impl_374 (impl)
macro_rules! Depcrate_read_endian_readerimpl_374 {
() => {
// Module: crate::read::endian_reader
// Provides: {"impl_374"}
// Dependencies: {}
impl < T > SubRange < T > where T : CloneStableDeref < Target = [u8] > + Debug , { # [inline] fn new (bytes : T) -> Self { let ptr = bytes . as_ptr () ; let len = bytes . len () ; SubRange { bytes , ptr , len } } # [inline] fn bytes (& self) -> & [u8] { unsafe { slice :: from_raw_parts (self . ptr , self . len) } } # [inline] fn len (& self) -> usize { self . len } # [inline] fn truncate (& mut self , len : usize) { assert ! (len <= self . len) ; self . len = len ; } # [inline] fn skip (& mut self , len : usize) { assert ! (len <= self . len) ; self . ptr = unsafe { self . ptr . add (len) } ; self . len -= len ; } # [inline] fn read_slice (& mut self , len : usize) -> Option < & [u8] > { if self . len () < len { None } else { let bytes = unsafe { slice :: from_raw_parts (self . ptr , len) } ; self . skip (len) ; Some (bytes) } } }
};
}
