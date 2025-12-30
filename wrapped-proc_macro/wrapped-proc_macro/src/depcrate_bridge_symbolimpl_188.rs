// Generated macro for impl_188 (impl)
macro_rules! Depcrate_bridge_symbolimpl_188 {
() => {
// Module: crate::bridge::symbol
// Provides: {"impl_188"}
// Dependencies: {}
impl Interner { fn intern (& mut self , string : & str) -> Symbol { if let Some (& name) = self . names . get (string) { return name ; } let name = Symbol (self . sym_base . checked_add (self . strings . len () as u32) . expect ("`proc_macro` symbol name overflow") ,) ; let string : & str = self . arena . alloc_str (string) ; let string : & 'static str = unsafe { & * (string as * const str) } ; self . strings . push (string) ; self . names . insert (string , name) ; name } # [doc = " Reads a symbol's value from the store while it is held."] fn get (& self , symbol : Symbol) -> & str { let name = symbol . 0 . get () . checked_sub (self . sym_base . get ()) . expect ("use-after-free of `proc_macro` symbol") ; self . strings [name as usize] } # [doc = " Clear all symbols from the store, invalidating them such that `get` will"] # [doc = " panic if they are accessed in the future."] fn clear (& mut self) { self . sym_base = self . sym_base . saturating_add (self . strings . len () as u32) ; self . names . clear () ; self . strings . clear () ; self . arena = arena :: Arena :: new () ; } }
};
}
