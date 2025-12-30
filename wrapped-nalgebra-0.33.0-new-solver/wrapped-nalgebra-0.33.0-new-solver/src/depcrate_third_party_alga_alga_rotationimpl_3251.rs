// Generated macro for impl_3251 (impl)
macro_rules! Depcrate_third_party_alga_alga_rotationimpl_3251 {
() => {
// Module: crate::third_party::alga::alga_rotation
// Provides: {"impl_3251"}
// Dependencies: {}
# [doc = " Subgroups of the n-dimensional rotation group `SO(n)`."] impl < T : RealField + simba :: scalar :: RealField , const D : usize > linear :: Rotation < Point < T , D > > for Rotation < T , D > { # [inline] fn powf (& self , _ : T) -> Option < Self > { unimplemented ! () } # [inline] fn rotation_between (_ : & SVector < T , D > , _ : & SVector < T , D >) -> Option < Self > { unimplemented ! () } # [inline] fn scaled_rotation_between (_ : & SVector < T , D > , _ : & SVector < T , D > , _ : T) -> Option < Self > { unimplemented ! () } }
};
}
