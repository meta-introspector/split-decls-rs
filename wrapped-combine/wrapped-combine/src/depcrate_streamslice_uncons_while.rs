// Generated macro for slice_uncons_while (function)
macro_rules! Depcrate_streamslice_uncons_while {
() => {
// Module: crate::stream
// Provides: {"slice_uncons_while"}
// Dependencies: {}
fn slice_uncons_while < 'a , T , F > (slice : & mut & 'a [T] , start : UnconsStart , mut f : F) -> & 'a [T] where F : FnMut (T) -> bool , T : Clone , { let mut i = start as usize ; let len = slice . len () ; debug_assert ! (len >= i , "") ; let mut found = false ; macro_rules ! check { () => { if ! f (unsafe { slice . get_unchecked (i) . clone () }) { found = true ; break ; } i += 1 ; } ; } while len - i >= 8 { check ! () ; check ! () ; check ! () ; check ! () ; check ! () ; check ! () ; check ! () ; check ! () ; } if ! found { while let Some (c) = slice . get (i) { if ! f (c . clone ()) { break ; } i += 1 ; } } let (result , remaining) = slice . split_at (i) ; * slice = remaining ; result }
};
}
