// Generated macro for impl_32 (impl)
macro_rules! Depcrate_modelimpl_32 {
() => {
// Module: crate::model
// Provides: {"impl_32"}
// Dependencies: {}
impl Model for Sphere { fn hit < 'a > (& 'a self , r : & Ray) -> Option < Hit < 'a > > { let oc = r . origin - self . center ; let a = r . direction . dot (r . direction) ; let hb = oc . dot (r . direction) ; let c = oc . dot (oc) - self . radius * self . radius ; let discriminant = hb * hb - a * c ; if discriminant > 0.0 { let t = (- hb - discriminant . sqrt ()) / a ; if t >= T_MIN { let p = r . point_at_parameter (t) ; return Some (Hit { t : t , p : p , normal : (p - self . center) / self . radius , material : & * self . material , }) ; } let t = (- hb + discriminant . sqrt ()) / a ; if t >= T_MIN { let p = r . point_at_parameter (t) ; return Some (Hit { t : t , p : p , normal : (p - self . center) / self . radius , material : & * self . material , }) ; } } None } }
};
}
