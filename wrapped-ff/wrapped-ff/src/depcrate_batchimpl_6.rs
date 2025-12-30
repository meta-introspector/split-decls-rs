// Generated macro for impl_6 (impl)
macro_rules! Depcrate_batchimpl_6 {
() => {
// Module: crate::batch
// Provides: {"impl_6"}
// Dependencies: {}
# [cfg (feature = "alloc")] # [cfg_attr (docsrs , doc (cfg (feature = "alloc")))] impl < 'a , F , I > BatchInvert < F > for I where F : Field + ConstantTimeEq , I : IntoIterator < Item = & 'a mut F > , { fn batch_invert (self) -> F { let mut acc = F :: ONE ; let iter = self . into_iter () ; let mut tmp = alloc :: vec :: Vec :: with_capacity (iter . size_hint () . 0) ; for p in iter { let q = * p ; tmp . push ((acc , p)) ; acc = F :: conditional_select (& (acc * q) , & acc , q . is_zero ()) ; } acc = acc . invert () . unwrap () ; let allinv = acc ; for (tmp , p) in tmp . into_iter () . rev () { let skip = p . is_zero () ; let tmp = tmp * acc ; acc = F :: conditional_select (& (acc * * p) , & acc , skip) ; * p = F :: conditional_select (& tmp , p , skip) ; } allinv } }
};
}
