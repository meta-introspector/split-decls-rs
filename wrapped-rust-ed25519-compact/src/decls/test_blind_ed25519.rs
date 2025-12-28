macro_rules! deps {
    () => {
        KeyPair!();
        Seed!();
    };
}

macro_rules! test_blind_ed25519 {
    () => {
        deps!();
        # [test] # [cfg (all (feature = "blind-keys" , feature = "random"))] fn test_blind_ed25519 () { use ct_codecs :: { Decoder , Hex } ; let kp = KeyPair :: generate () ; let blind = Blind :: new ([69u8 ; 32]) ; let blind_kp = kp . blind (& blind , "ctx") ; let message = b"Hello, World!" ; let signature = blind_kp . blind_sk . sign (message , None) ; assert ! (blind_kp . blind_pk . verify (message , & signature) . is_ok ()) ; let recovered_pk = blind_kp . blind_pk . unblind (& blind , "ctx") . unwrap () ; assert ! (recovered_pk == kp . pk) ; let kp = KeyPair :: from_seed (Seed :: from_slice (& Hex :: decode_to_vec ("875532ab039b0a154161c284e19c74afa28d5bf5454e99284bbcffaa71eebf45" , None ,) . unwrap () ,) . unwrap () ,) ; assert_eq ! (Hex :: decode_to_vec ("3b5983605b277cd44918410eb246bb52d83adfc806ccaa91a60b5b2011bc5973" , None) . unwrap () , kp . pk . as_ref ()) ; let blind = Blind :: from_slice (& Hex :: decode_to_vec ("c461e8595f0ac41d374f878613206704978115a226f60470ffd566e9e6ae73bf" , None ,) . unwrap () ,) . unwrap () ; let blind_kp = kp . blind (& blind , "ctx") ; assert_eq ! (Hex :: decode_to_vec ("246dcd43930b81d5e4d770db934a9fcd985b75fd014bc2a98b0aea02311c1836" , None) . unwrap () , blind_kp . blind_pk . as_ref ()) ; let message = Hex :: decode_to_vec ("68656c6c6f20776f726c64" , None) . unwrap () ; let signature = blind_kp . blind_sk . sign (message , None) ; assert_eq ! (Hex :: decode_to_vec ("947bacfabc63448f8955dc20630e069e58f37b72bb433ae17f2fa904ea860b44deb761705a3cc2168a6673ee0b41ff7765c7a4896941eec6833c1689315acb0b" , None) . unwrap () , signature . as_ref ()) ; }
    };
}

test_blind_ed25519!()