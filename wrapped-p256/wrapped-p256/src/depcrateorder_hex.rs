// Generated macro for ORDER_HEX (const)
macro_rules! DepcrateORDER_HEX {
() => {
// Module: crate
// Provides: {"ORDER_HEX"}
// Dependencies: {}
# [doc = " Order of NIST P-256's elliptic curve group (i.e. scalar modulus) serialized"] # [doc = " as hexadecimal."] # [doc = ""] # [doc = " ```text"] # [doc = " n = FFFFFFFF 00000000 FFFFFFFF FFFFFFFF BCE6FAAD A7179E84 F3B9CAC2 FC632551"] # [doc = " ```"] # [doc = ""] # [doc = " # Calculating the order"] # [doc = " One way to calculate the order is with `GP/PARI`:"] # [doc = ""] # [doc = " ```text"] # [doc = " p = (2^224) * (2^32 - 1) + 2^192 + 2^96 - 1"] # [doc = " b = 41058363725152142129326129780047268409114441015993725554835256314039467401291"] # [doc = " E = ellinit([Mod(-3, p), Mod(b, p)])"] # [doc = " default(parisize, 120000000)"] # [doc = " n = ellsea(E)"] # [doc = " isprime(n)"] # [doc = " ```"] const ORDER_HEX : & str = "ffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632551" ;
};
}
