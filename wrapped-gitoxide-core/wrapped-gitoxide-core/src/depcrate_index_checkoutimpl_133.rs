// Generated macro for impl_133 (impl)
macro_rules! Depcrate_index_checkoutimpl_133 {
() => {
// Module: crate::index::checkout
// Provides: {"impl_133"}
// Dependencies: {}
impl < Find > gix :: objs :: Find for EmptyOrDb < Find > where Find : gix :: objs :: Find , { fn try_find < 'a > (& self , id : & gix :: oid , buf : & 'a mut Vec < u8 >) -> Result < Option < gix :: objs :: Data < 'a > > , Error > { if self . empty_files { let Some (kind) = self . db . try_find (id , buf) ? . map (| d | d . kind) else { return Ok (None) ; } ; buf . clear () ; Ok (Some (gix :: objs :: Data { kind , data : buf })) } else { self . db . try_find (id , buf) } } }
};
}
