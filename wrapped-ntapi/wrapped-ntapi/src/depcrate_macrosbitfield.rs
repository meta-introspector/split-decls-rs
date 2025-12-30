// Generated macro for BITFIELD (macro)
macro_rules! Depcrate_macrosBITFIELD {
() => {
// Module: crate::macros
// Provides: {"BITFIELD"}
// Dependencies: {}
macro_rules ! BITFIELD { ($ base : ident $ field : ident : $ fieldtype : ty [$ ($ thing : ident $ set_thing : ident [$ r : expr] ,) +]) => { impl $ base { $ (# [inline] pub const fn $ thing (& self) -> $ fieldtype { const SIZE : usize = $ crate :: _core :: mem :: size_of ::<$ fieldtype > () * 8 ; self .$ field << (SIZE - $ r . end) >> (SIZE - $ r . end + $ r . start) } # [inline] pub fn $ set_thing (& mut self , val : $ fieldtype) { const MASK : $ fieldtype = ((1 << ($ r . end - $ r . start)) - 1) << $ r . start ; self .$ field &= ! MASK ; self .$ field |= (val << $ r . start) & MASK ; }) + } } ; (unsafe $ base : ident $ field : ident : $ fieldtype : ty [$ ($ thing : ident $ set_thing : ident [$ r : expr] ,) +]) => { impl $ base { $ (# [inline] pub unsafe fn $ thing (& self) -> $ fieldtype { const SIZE : usize = $ crate :: _core :: mem :: size_of ::<$ fieldtype > () * 8 ; self .$ field << (SIZE - $ r . end) >> (SIZE - $ r . end + $ r . start) } # [inline] pub unsafe fn $ set_thing (& mut self , val : $ fieldtype) { const MASK : $ fieldtype = ((1 << ($ r . end - $ r . start)) - 1) << $ r . start ; self .$ field &= ! MASK ; self .$ field |= (val << $ r . start) & MASK ; }) + } } ; }
};
}
