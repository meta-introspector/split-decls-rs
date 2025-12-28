macro_rules! deps {
    () => {
        Group!();
        GroupOps!();
        GroupOpsOwned!();
    };
}

macro_rules! Curve {
    () => {
        deps!();
        # [doc = " Efficient representation of an elliptic curve point guaranteed."] pub trait Curve : Group + GroupOps < < Self as Curve > :: AffineRepr > + GroupOpsOwned < < Self as Curve > :: AffineRepr > { # [doc = " The affine representation for this elliptic curve."] type AffineRepr ; # [doc = " Converts a batch of projective elements into affine elements. This function will"] # [doc = " panic if `p.len() != q.len()`."] fn batch_normalize (p : & [Self] , q : & mut [Self :: AffineRepr]) { assert_eq ! (p . len () , q . len ()) ; for (p , q) in p . iter () . zip (q . iter_mut ()) { * q = p . to_affine () ; } } # [doc = " Converts this element into its affine representation."] fn to_affine (& self) -> Self :: AffineRepr ; }
    };
}

Curve!()