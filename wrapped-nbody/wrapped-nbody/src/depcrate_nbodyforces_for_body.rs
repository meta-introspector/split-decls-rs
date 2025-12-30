// Generated macro for forces_for_body (function)
macro_rules! Depcrate_nbodyforces_for_body {
() => {
// Module: crate::nbody
// Provides: {"forces_for_body"}
// Dependencies: {}
# [doc = " Given the position and mass of one body,"] # [doc = " calculate the force acting on it from all of the other bodies."] fn forces_for_body < 'a , I > (p : & Position , m : Number , reference : I) -> Force where I : IntoIterator < Item = (& 'a Position , & 'a Number) > , { reference . into_iter () . map (| (ref otherpos , & othermass) | { let dx = p . x - otherpos . x ; let dy = p . y - otherpos . y ; let dz = p . z - otherpos . z ; let d = dist_squared (dx , dy , dz) ; let f = force_d (m , othermass , d) ; Force { fx : (f * dx) / d , fy : (f * dy) / d , fz : (f * dz) / d , } }) . reduce (| acc : Force , f : Force | Force { fx : acc . fx + f . fx , fy : acc . fy + f . fy , fz : acc . fz + f . fz , }) . unwrap () }
};
}
