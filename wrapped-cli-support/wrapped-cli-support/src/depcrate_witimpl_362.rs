// Generated macro for impl_362 (impl)
macro_rules! Depcrate_witimpl_362 {
() => {
// Module: crate::wit
// Provides: {"impl_362"}
// Dependencies: {}
impl StructUnpacker { fn new () -> Self { Self { next_offset : 0 } } fn align_up (& mut self , alignment_pow2 : usize) -> usize { let mask = alignment_pow2 - 1 ; self . next_offset = (self . next_offset + mask) & (! mask) ; self . next_offset } fn append (& mut self , quads : usize , alignment_pow2 : usize) -> usize { let ret = self . align_up (alignment_pow2) ; self . next_offset += quads ; ret } # [doc = " Returns the offset for this member, with the offset in multiples of u32."] fn read_ty (& mut self , ty : & AdapterType) -> Result < usize , Error > { let (quads , alignment) = match ty { AdapterType :: I32 | AdapterType :: U32 | AdapterType :: F32 => (1 , 1) , AdapterType :: I64 | AdapterType :: U64 | AdapterType :: F64 => (2 , 2) , other => bail ! ("invalid aggregate return type {other:?}") , } ; Ok (self . append (quads , alignment)) } }
};
}
