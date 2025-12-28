macro_rules! deps {
    () => {
        VecType!();
    };
}

macro_rules! Bigint {
    () => {
        deps!();
        # [doc = " Storage for a big integer type."] # [doc = ""] # [doc = " This is used for algorithms when we have a finite number of digits."] # [doc = " Specifically, it stores all the significant digits scaled to the"] # [doc = " proper exponent, as an integral type, and then directly compares"] # [doc = " these digits."] # [doc = ""] # [doc = " This requires us to store the number of significant bits, plus the"] # [doc = " number of exponent bits (required) since we scale everything"] # [doc = " to the same exponent."] # [derive (Clone , PartialEq , Eq)] pub struct Bigint { # [doc = " Significant digits for the float, stored in a big integer in LE order."] # [doc = ""] # [doc = " This is pretty much the same number of digits for any radix, since the"] # [doc = "  significant digits balances out the zeros from the exponent:"] # [doc = "     1. Decimal is 1091 digits, 767 mantissa digits + 324 exponent zeros."] # [doc = "     2. Base 6 is 1097 digits, or 680 mantissa digits + 417 exponent zeros."] # [doc = "     3. Base 36 is 1086 digits, or 877 mantissa digits + 209 exponent zeros."] # [doc = ""] # [doc = " However, the number of bytes required is larger for large radixes:"] # [doc = " for decimal, we need `log2(10**1091) ≅ 3600`, while for base 36"] # [doc = " we need `log2(36**1086) ≅ 5600`. Since we use uninitialized data,"] # [doc = " we avoid a major performance hit from the large buffer size."] pub data : VecType , }
    };
}

Bigint!()