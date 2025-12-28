macro_rules! deps {
    () => {
        Signature!();
        VerifyingKey!();
    };
}

macro_rules! new_verification_test {
    () => {
        deps!();
        # [doc = " Define ECDSA verification test."] # [macro_export] macro_rules ! new_verification_test { ($ curve : path , $ vectors : expr) => { use $ crate :: { Signature , VerifyingKey , elliptic_curve :: { AffinePoint , CurveArithmetic , Scalar , array :: Array , group :: ff :: PrimeField , sec1 :: { EncodedPoint , FromEncodedPoint } , } , signature :: hazmat :: PrehashVerifier , } ; # [test] fn ecdsa_verify_success () { for vector in $ vectors { let q_encoded = EncodedPoint ::<$ curve >:: from_affine_coordinates (& Array :: try_from (vector . q_x) . unwrap () , & Array :: try_from (vector . q_y) . unwrap () , false ,) ; let q = VerifyingKey ::<$ curve >:: from_encoded_point (& q_encoded) . unwrap () ; let sig = Signature :: from_scalars (Array :: try_from (vector . r) . unwrap () , Array :: try_from (vector . s) . unwrap () ,) . unwrap () ; let result = q . verify_prehash (vector . m , & sig) ; assert ! (result . is_ok ()) ; } } # [test] fn ecdsa_verify_invalid_s () { for vector in $ vectors { let q_encoded = EncodedPoint ::<$ curve >:: from_affine_coordinates (& Array :: try_from (vector . q_x) . unwrap () , & Array :: try_from (vector . q_y) . unwrap () , false ,) ; let q = VerifyingKey ::<$ curve >:: from_encoded_point (& q_encoded) . unwrap () ; let r = Array :: try_from (vector . r) . unwrap () ; let mut s_tweaked = Array :: try_from (vector . s) . unwrap () ; s_tweaked [0] ^= 1 ; let sig = Signature :: from_scalars (r , s_tweaked) . unwrap () ; let result = q . verify_prehash (vector . m , & sig) ; assert ! (result . is_err ()) ; } } } ; }
    };
}

new_verification_test!()