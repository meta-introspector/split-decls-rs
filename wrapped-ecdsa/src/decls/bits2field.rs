macro_rules! deps {
    () => {
        EcdsaCurve!();
    };
}

macro_rules! bits2field {
    () => {
        deps!();
        # [doc = " Partial implementation of the `bits2int` function as defined in"] # [doc = " [RFC6979 § 2.3.2] as well as [SEC1] § 2.3.8."] # [doc = ""] # [doc = " This is used to convert a message digest whose size may be smaller or"] # [doc = " larger than the size of the curve's scalar field into a serialized"] # [doc = " (unreduced) field element."] # [doc = ""] # [doc = " [RFC6979 § 2.3.2]: https://datatracker.ietf.org/doc/html/rfc6979#section-2.3.2"] # [doc = " [SEC1]: https://www.secg.org/sec1-v2.pdf"] pub fn bits2field < C : EcdsaCurve > (bits : & [u8]) -> Result < FieldBytes < C > > { if bits . len () < C :: FieldBytesSize :: USIZE / 2 { return Err (Error :: new ()) ; } let mut field_bytes = FieldBytes :: < C > :: default () ; match bits . len () . cmp (& C :: FieldBytesSize :: USIZE) { cmp :: Ordering :: Equal => field_bytes . copy_from_slice (bits) , cmp :: Ordering :: Less => { field_bytes [(C :: FieldBytesSize :: USIZE - bits . len ()) ..] . copy_from_slice (bits) ; } cmp :: Ordering :: Greater => { field_bytes . copy_from_slice (& bits [.. C :: FieldBytesSize :: USIZE]) ; } } Ok (field_bytes) }
    };
}

bits2field!()