// Generated macro for pin_as_deref_mut (function)
macro_rules! Depcrate_rt_iopin_as_deref_mut {
() => {
// Module: crate::rt::io
// Provides: {"pin_as_deref_mut"}
// Dependencies: {}
# [doc = " Polyfill for Pin::as_deref_mut()"] # [doc = " TODO: use Pin::as_deref_mut() instead once stabilized"] fn pin_as_deref_mut < P : DerefMut > (pin : Pin < & mut Pin < P > >) -> Pin < & mut P :: Target > { unsafe { pin . get_unchecked_mut () } . as_mut () }
};
}
