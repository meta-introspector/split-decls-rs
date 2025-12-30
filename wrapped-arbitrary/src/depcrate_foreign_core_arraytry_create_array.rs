// Generated macro for try_create_array (function)
macro_rules! Depcrate_foreign_core_arraytry_create_array {
() => {
// Module: crate::foreign::core::array
// Provides: {"try_create_array"}
// Dependencies: {}
fn try_create_array < F , T , const N : usize > (mut cb : F) -> Result < [T ; N] > where F : FnMut (usize) -> Result < T > , { let mut array : MaybeUninit < [T ; N] > = MaybeUninit :: uninit () ; let array_ptr = array . as_mut_ptr () ; let dst = array_ptr as _ ; let mut guard : ArrayGuard < T , N > = ArrayGuard { dst , initialized : 0 , } ; unsafe { for (idx , value_ptr) in (* array . as_mut_ptr ()) . iter_mut () . enumerate () { ptr :: write (value_ptr , cb (idx) ?) ; guard . initialized += 1 ; } mem :: forget (guard) ; Ok (array . assume_init ()) } }
};
}
