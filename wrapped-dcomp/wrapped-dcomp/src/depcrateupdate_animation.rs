// Generated macro for update_animation (function)
macro_rules! Depcrateupdate_animation {
() => {
// Module: crate
// Provides: {"update_animation"}
// Dependencies: {}
fn update_animation (device : & IDCompositionDesktopDevice , card : & Card) -> Result < () > { unsafe { let animation = device . CreateAnimation () ? ; card . variable . GetCurve (& animation) ? ; card . rotation . as_ref () . expect ("IDCompositionRotateTransform3D") . SetAngle (& animation) } }
};
}
