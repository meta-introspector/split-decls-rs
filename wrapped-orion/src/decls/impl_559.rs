macro_rules! deps {
    () => {
        DecapsulationKey!();
        MlKem768!();
        XWing!();
        UnknownCryptoError!();
        PublicKey!();
        Sha3_256!();
        PrivateKey!();
        EncapsulationKey!();
    };
}

macro_rules! impl_559 {
    () => {
        deps!();
        impl XWing { const LABEL : & [u8 ; 6] = b"\\.//^\\" ; fn combiner (ss_m : & [u8] , ss_x : & [u8] , ct_x : & [u8] , pk_x : & [u8] ,) -> Result < SharedSecret , UnknownCryptoError > { let mut ctx = sha3_256 :: Sha3_256 :: new () ; ctx . update (ss_m) ? ; ctx . update (ss_x) ? ; ctx . update (ct_x) ? ; ctx . update (pk_x) ? ; ctx . update (Self :: LABEL) ? ; let mut digest = Zeroizing :: new ([0u8 ; 32]) ; ctx . _finalize_internal (digest . as_mut ()) ? ; Ok (SharedSecret :: from (* digest)) } # [doc = " Given the [EncapsulationKey] and securely generated randomness `eseed`, generate a [SharedSecret] and associated [Ciphertext]."] pub fn encap_deterministic (ek : & EncapsulationKey , eseed : & [u8] ,) -> Result < (SharedSecret , Ciphertext) , UnknownCryptoError > { if eseed . len () != 64 { return Err (UnknownCryptoError) ; } let pk_m = & ek . as_ref () [.. mlkem768 :: MlKem768 :: EK_SIZE] ; let pk_x = & ek . as_ref () [mlkem768 :: MlKem768 :: EK_SIZE ..] ; let ek_x = x25519 :: PrivateKey :: from_slice (& eseed [32 .. 64]) ? ; let ct_x = x25519 :: PublicKey :: try_from (& ek_x) ? . to_bytes () ; let ss_x = x25519 :: key_agreement (& ek_x , & x25519 :: PublicKey :: from_slice (pk_x) ?) ? ; let mlkem768_encapkey = mlkem768 :: EncapsulationKey :: from_slice (pk_m) ? ; let (ss_m , ct_m) = mlkem768_encapkey . encap_deterministic (& eseed [.. 32]) ? ; let ss = Self :: combiner (ss_m . unprotected_as_bytes () , ss_x . unprotected_as_bytes () , & ct_x , pk_x ,) ? ; let mut ct = [0u8 ; mlkem768 :: MlKem768 :: CIPHERTEXT_SIZE + x25519 :: PUBLIC_KEY_SIZE] ; ct [.. mlkem768 :: MlKem768 :: CIPHERTEXT_SIZE] . copy_from_slice (ct_m . as_ref ()) ; ct [mlkem768 :: MlKem768 :: CIPHERTEXT_SIZE ..] . copy_from_slice (& ct_x) ; Ok ((ss , Ciphertext :: from (ct))) } # [cfg (feature = "safe_api")] # [cfg_attr (docsrs , doc (cfg (feature = "safe_api")))] # [doc = " Given the [EncapsulationKey], generate a [SharedSecret] and associated [Ciphertext]."] pub fn encap (ek : & EncapsulationKey) -> Result < (SharedSecret , Ciphertext) , UnknownCryptoError > { let mut eseed = Zeroizing :: new ([0u8 ; 64]) ; getrandom :: fill (eseed . as_mut ()) ? ; Self :: encap_deterministic (ek , eseed . as_ref ()) } # [doc = " Given the [DecapsulationKey], produce a [SharedSecret] using the [Ciphertext]."] pub fn decap (dk : & DecapsulationKey , c : & Ciphertext ,) -> Result < SharedSecret , UnknownCryptoError > { let ct_m = & c . as_ref () [.. mlkem768 :: MlKem768 :: CIPHERTEXT_SIZE] ; let ct_x = & c . as_ref () [mlkem768 :: MlKem768 :: CIPHERTEXT_SIZE ..] ; let ss_m = mlkem768 :: MlKem768 :: decap (dk . kp_m . private () , & mlkem768 :: Ciphertext :: from_slice (ct_m) ?) ? ; let ss_x = x25519 :: key_agreement (& dk . sk_x , & x25519 :: PublicKey :: from_slice (ct_x) ?) ? ; Self :: combiner (ss_m . unprotected_as_bytes () , ss_x . unprotected_as_bytes () , ct_x , & dk . pk_x . to_bytes () ,) } }
    };
}

impl_559!()