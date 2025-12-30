// Generated macro for push_interp (function)
macro_rules! Depcratepush_interp {
() => {
// Module: crate
// Provides: {"push_interp"}
// Dependencies: {}
fn push_interp < X : Clone , Y : Clone > (collection : & mut Vec < (X , Y) > , value : (X , Y)) { let prev = collection . last () . cloned () ; let new_time = value . 0 . clone () ; if let Some ((_ , y)) = prev { collection . push ((new_time , y)) ; } collection . push (value) }
};
}
