macro_rules! deps {
    () => {
        Signature!();
    };
}

macro_rules! TestVector {
    () => {
        deps!();
        # [doc = " ECDSA test vector"] pub struct TestVector { # [doc = " Private scalar"] pub d : & 'static [u8] , # [doc = " Public key x-coordinate (`Qx`)"] pub q_x : & 'static [u8] , # [doc = " Public key y-coordinate (`Qy`)"] pub q_y : & 'static [u8] , # [doc = " Ephemeral scalar (a.k.a. nonce)"] pub k : & 'static [u8] , # [doc = " Message digest (prehashed)"] pub m : & 'static [u8] , # [doc = " Signature `r` component"] pub r : & 'static [u8] , # [doc = " Signature `s` component"] pub s : & 'static [u8] , }
    };
}

TestVector!();