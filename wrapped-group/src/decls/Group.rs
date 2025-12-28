macro_rules! deps {
    () => {
        ScalarMul!();
        GroupOpsOwned!();
        ScalarMulOwned!();
        GroupOps!();
    };
}

macro_rules! Group {
    () => {
        deps!();
        # [doc = " This trait represents an element of a cryptographic group."] pub trait Group : Clone + Copy + fmt :: Debug + Eq + Sized + Send + Sync + 'static + Sum + for < 'a > Sum < & 'a Self > + Neg < Output = Self > + GroupOps + GroupOpsOwned + ScalarMul < < Self as Group > :: Scalar > + ScalarMulOwned < < Self as Group > :: Scalar > { # [doc = " Scalars modulo the order of this group's scalar field."] type Scalar : PrimeField ; # [doc = " Returns an element chosen uniformly at random from the non-identity elements of"] # [doc = " this group."] # [doc = ""] # [doc = " This function is non-deterministic, and samples from the user-provided RNG."] fn random (rng : impl RngCore) -> Self ; # [doc = " Returns the additive identity, also known as the \"neutral element\"."] fn identity () -> Self ; # [doc = " Returns a fixed generator of the prime-order subgroup."] fn generator () -> Self ; # [doc = " Determines if this point is the identity."] fn is_identity (& self) -> Choice ; # [doc = " Doubles this element."] # [must_use] fn double (& self) -> Self ; }
    };
}

Group!();