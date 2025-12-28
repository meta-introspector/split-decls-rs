macro_rules! deps {
    () => {
        KeyPair!();
    };
}

macro_rules! test_ed25519 {
    () => {
        deps!();
        # [test] fn test_ed25519 () { let kp = KeyPair :: from_seed ([42u8 ; 32] . into ()) ; let message = b"Hello, World!" ; let signature = kp . sk . sign (message , None) ; assert ! (kp . pk . verify (message , & signature) . is_ok ()) ; assert ! (kp . pk . verify (b"Hello, world!" , & signature) . is_err ()) ; assert_eq ! (signature . as_ref () , [196 , 182 , 1 , 15 , 182 , 182 , 231 , 166 , 227 , 62 , 243 , 85 , 49 , 174 , 169 , 9 , 162 , 196 , 98 , 104 , 30 , 81 , 22 , 38 , 184 , 136 , 253 , 128 , 10 , 160 , 128 , 105 , 127 , 130 , 138 , 164 , 57 , 86 , 94 , 160 , 216 , 85 , 153 , 139 , 81 , 100 , 38 , 124 , 235 , 210 , 26 , 95 , 231 , 90 , 73 , 206 , 33 , 216 , 171 , 15 , 188 , 181 , 136 , 7 ,]) ; }
    };
}

test_ed25519!()