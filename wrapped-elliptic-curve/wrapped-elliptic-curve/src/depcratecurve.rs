// Generated macro for Curve (trait)
macro_rules! DepcrateCurve {
() => {
// Module: crate
// Provides: {"Curve"}
// Dependencies: {}
# [doc = " Elliptic curve."] # [doc = ""] # [doc = " This trait is intended to be impl'd by a ZST which represents a concrete elliptic curve."] # [doc = ""] # [doc = " Other traits in this crate which are bounded by [`Curve`] are intended to be impl'd by these"] # [doc = " ZSTs, facilitating types which are generic over elliptic curves (e.g. [`SecretKey`])."] pub trait Curve : 'static + Copy + Clone + Debug + Default + Eq + Ord + Send + Sync { # [doc = " Size of a serialized field element in bytes."] # [doc = ""] # [doc = " This is typically the same as `Self::Uint::ByteSize` but for curves"] # [doc = " with an unusual field modulus (e.g. P-224, P-521) it may be different."] type FieldBytesSize : ArraySize + Add + Eq ; # [doc = " Integer type used to represent field elements of this elliptic curve."] type Uint : bigint :: ArrayEncoding + bigint :: Encoding + bigint :: FixedInteger + bigint :: Random + bigint :: RandomMod + bigint :: Unsigned + zeroize :: Zeroize + FieldBytesEncoding < Self > + ShrAssign < usize > ; # [doc = " Order of this curve's prime order subgroup, i.e. number of elements in the scalar field."] const ORDER : Odd < Self :: Uint > ; }
};
}
