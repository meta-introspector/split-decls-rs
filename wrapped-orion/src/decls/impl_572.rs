macro_rules! deps {
    () => {
        UnknownCryptoError!();
        Role!();
        PublicKey!();
        ModeAuth!();
        PrivateKey!();
    };
}

macro_rules! impl_572 {
    () => {
        deps!();
        impl < S : Suite + Auth > ModeAuth < S > { # [cfg (feature = "safe_api")] # [cfg_attr (docsrs , doc (cfg (feature = "safe_api")))] # [doc = " HPKE Auth mode sender."] pub fn new_sender (pubkey_r : & S :: PublicKey , info : & [u8] , secret_key_s : & S :: PrivateKey ,) -> Result < (Self , S :: EncapsulatedKey) , UnknownCryptoError > { let (suite , ek) = S :: setup_auth_sender (pubkey_r , info , secret_key_s) ? ; Ok (((Self { suite , role : Role :: Sender , }) , ek ,)) } # [doc = " HPKE Auth mode sender with a supplied ephemeral private key, which is taken ownership of."] pub fn new_sender_deterministic (pubkey_r : & S :: PublicKey , info : & [u8] , secret_key_s : & S :: PrivateKey , secret_ephemeral : S :: PrivateKey ,) -> Result < (Self , S :: EncapsulatedKey) , UnknownCryptoError > { let (suite , ek) = S :: setup_auth_sender_deterministic (pubkey_r , info , secret_key_s , secret_ephemeral) ? ; Ok (((Self { suite , role : Role :: Sender , }) , ek ,)) } # [doc = " HPKE Auth mode recipient."] pub fn new_recipient (enc : & S :: EncapsulatedKey , secret_key_r : & S :: PrivateKey , info : & [u8] , pubkey_s : & S :: PublicKey ,) -> Result < Self , UnknownCryptoError > { Ok (Self { suite : S :: setup_auth_recipient (enc , secret_key_r , info , pubkey_s) ? , role : Role :: Recipient , }) } # [doc = " Context-aware sealing operations."] pub fn seal (& mut self , plaintext : & [u8] , aad : & [u8] , out : & mut [u8] ,) -> Result < () , UnknownCryptoError > { if self . role != Role :: Sender { return Err (UnknownCryptoError) ; } self . suite . seal (plaintext , aad , out) } # [doc = " Context-aware opening operations."] pub fn open (& mut self , ciphertext : & [u8] , aad : & [u8] , out : & mut [u8] ,) -> Result < () , UnknownCryptoError > { if self . role != Role :: Recipient { return Err (UnknownCryptoError) ; } self . suite . open (ciphertext , aad , out) } # [cfg (feature = "safe_api")] # [cfg_attr (docsrs , doc (cfg (feature = "safe_api")))] # [doc = " One-shot API for HPKE Auth mode [`Self::seal()`] operation."] pub fn auth_seal (pubkey_r : & S :: PublicKey , info : & [u8] , secrety_key_s : & S :: PrivateKey , plaintext : & [u8] , aad : & [u8] , out : & mut [u8] ,) -> Result < S :: EncapsulatedKey , UnknownCryptoError > { let (mut ctx , ek) = Self :: new_sender (pubkey_r , info , secrety_key_s) ? ; ctx . seal (plaintext , aad , out) ? ; Ok (ek) } # [doc = " One-shot API for HPKE Auth mode [`Self::open()`] operation."] pub fn auth_open (enc : & S :: EncapsulatedKey , secret_key_r : & S :: PrivateKey , info : & [u8] , pubkey_s : & S :: PublicKey , ciphertext : & [u8] , aad : & [u8] , out : & mut [u8] ,) -> Result < () , UnknownCryptoError > { let mut ctx = Self :: new_recipient (enc , secret_key_r , info , pubkey_s) ? ; ctx . open (ciphertext , aad , out) } # [doc = " Export secret."] pub fn export_secret (& self , exporter_context : & [u8] , out : & mut [u8] ,) -> Result < () , UnknownCryptoError > { self . suite . export (exporter_context , out) } }
    };
}

impl_572!();