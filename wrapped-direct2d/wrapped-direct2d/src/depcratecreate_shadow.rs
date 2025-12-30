// Generated macro for create_shadow (function)
macro_rules! Depcratecreate_shadow {
() => {
// Module: crate
// Provides: {"create_shadow"}
// Dependencies: {}
fn create_shadow (target : & ID2D1DeviceContext , clock : & ID2D1Bitmap1) -> Result < ID2D1Effect > { unsafe { let shadow = target . CreateEffect (& CLSID_D2D1Shadow) ? ; shadow . SetInput (0 , clock , true) ; Ok (shadow) } }
};
}
