macro_rules! deps {
    () => {
        GroupOpsOwned!();
        PrimeGroup!();
        GroupEncoding!();
        GroupOps!();
        Group!();
    };
}

macro_rules! CofactorGroup {
    () => {
        deps!();
        # [doc = " This trait represents an element of a cryptographic group with a large prime-order"] # [doc = " subgroup and a comparatively-small cofactor."] pub trait CofactorGroup : Group + GroupEncoding + GroupOps < < Self as CofactorGroup > :: Subgroup > + GroupOpsOwned < < Self as CofactorGroup > :: Subgroup > { # [doc = " The large prime-order subgroup in which cryptographic operations are performed."] # [doc = " If `Self` implements `PrimeGroup`, then `Self::Subgroup` may be `Self`."] type Subgroup : PrimeGroup < Scalar = Self :: Scalar > + Into < Self > ; # [doc = " Maps `self` to the prime-order subgroup by multiplying this element by some"] # [doc = " `k`-multiple of the cofactor."] # [doc = ""] # [doc = " The value `k` does not vary between inputs for a given implementation, but may"] # [doc = " vary between different implementations of `CofactorGroup` because some groups have"] # [doc = " more efficient methods of clearing the cofactor when `k` is allowed to be"] # [doc = " different than `1`."] # [doc = ""] # [doc = " If `Self` implements [`PrimeGroup`], this returns `self`."] fn clear_cofactor (& self) -> Self :: Subgroup ; # [doc = " Returns `self` if it is contained in the prime-order subgroup."] # [doc = ""] # [doc = " If `Self` implements [`PrimeGroup`], this returns `Some(self)`."] fn into_subgroup (self) -> CtOption < Self :: Subgroup > ; # [doc = " Determines if this element is of small order."] # [doc = ""] # [doc = " Returns:"] # [doc = " - `true` if `self` is in the torsion subgroup."] # [doc = " - `false` if `self` is not in the torsion subgroup."] fn is_small_order (& self) -> Choice { self . clear_cofactor () . is_identity () } # [doc = " Determines if this element is \"torsion free\", i.e., is contained in the"] # [doc = " prime-order subgroup."] # [doc = ""] # [doc = " Returns:"] # [doc = " - `true` if `self` has trivial torsion and is in the prime-order subgroup."] # [doc = " - `false` if `self` has non-zero torsion component and is not in the prime-order"] # [doc = "   subgroup."] fn is_torsion_free (& self) -> Choice ; }
    };
}

CofactorGroup!()