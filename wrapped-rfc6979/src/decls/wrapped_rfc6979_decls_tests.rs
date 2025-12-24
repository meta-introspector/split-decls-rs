use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(test)]
mod tests {
    use crate::{
        Array, consts::{U21, U66},
        generate_k,
    };
    use hex_literal::hex;
    use sha2::{Digest, Sha256, Sha512};
    /// "Detailed Example" from RFC6979 Appendix A.1.
    ///
    /// Example for ECDSA on the curve K-163 described in FIPS 186-4 (also known as
    /// "ansix9t163k1" in X9.62), defined over a field GF(2^163)
    #[test]
    fn k163_sha256() {
        let q = hex!("04000000000000000000020108A2E0CC0D99F8A5EF");
        let x = hex!("009A4D6792295A7F730FC3F2B49CBC0F62E862272F");
        let h2 = hex!("01795EDF0D54DB760F156D0DAC04C0322B3A204224");
        let aad = b"";
        let k = generate_k::<Sha256, U21>(&x.into(), &q.into(), &h2.into(), aad);
        assert_eq!(k, hex!("023AF4074C90A02B3FE61D286D5C87F425E6BDD81B"));
    }
    /// Example from RFC6979 Appendix A.2.7.
    #[test]
    fn p521_sha512() {
        let q = hex!(
            "01FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFA51868783BF2F966B7FCC0148F709A5D03BB5C9B8899C47AEBB6FB71E91386409"
        );
        let x = hex!(
            "00FAD06DAA62BA3B25D2FB40133DA757205DE67F5BB0018FEE8C86E1B68C7E75CAA896EB32F1F47C70855836A6D16FCC1466F6D8FBEC67DB89EC0C08B0E996B83538"
        );
        let message = "sample";
        let mut h = Array::<u8, U66>::default();
        h[2..].copy_from_slice(&Sha512::digest(message));
        let aad = b"";
        let k = generate_k::<Sha512, U66>(&x.into(), &q.into(), &h, aad);
        let expected_k = hex!(
            "01DAE2EA071F8110DC26882D4D5EAE0621A3256FC8847FB9022E2B7D28E6F10198B1574FDD03A9053C08A1854A168AA5A57470EC97DD5CE090124EF52A2F7ECBFFD3"
        );
        assert_eq!(k, expected_k);
    }
}
