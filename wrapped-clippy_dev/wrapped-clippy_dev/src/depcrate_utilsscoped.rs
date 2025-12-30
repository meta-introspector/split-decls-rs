// Generated macro for Scoped (struct)
macro_rules! Depcrate_utilsScoped {
() => {
// Module: crate::utils
// Provides: {"Scoped"}
// Dependencies: {}
pub struct Scoped < 'inner , 'outer : 'inner , T > (T , PhantomData < & 'inner mut T > , PhantomData < & 'outer mut () >) ;
};
}
