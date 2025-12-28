macro_rules! deps {
    () => {
        UnknownCryptoError!();
    };
}

macro_rules! AeadTestRunner {
    () => {
        deps!();
        # [allow (clippy :: too_many_arguments)] # [cfg (feature = "safe_api")] # [doc = " Test runner for AEADs."] pub fn AeadTestRunner < Sealer , Opener , Key , Nonce > (sealer : Sealer , opener : Opener , key : Key , nonce : Nonce , input : & [u8] , expected_ct_with_tag : Option < & [u8] > , tag_size : usize , aad : & [u8] ,) where Sealer : Fn (& Key , & Nonce , & [u8] , Option < & [u8] > , & mut [u8]) -> Result < () , UnknownCryptoError > , Opener : Fn (& Key , & Nonce , & [u8] , Option < & [u8] > , & mut [u8]) -> Result < () , UnknownCryptoError > , { seal_dst_out_length (& sealer , & key , & nonce , input , tag_size , aad) ; open_dst_out_length (& sealer , & opener , & key , & nonce , input , tag_size , aad) ; open_modified_tag_err (& sealer , & opener , & key , & nonce , input , tag_size , aad) ; open_modified_ciphertext_err (& sealer , & opener , & key , & nonce , input , tag_size , aad) ; open_modified_aad_err (& sealer , & opener , & key , & nonce , input , tag_size , aad) ; none_or_empty_some_aad_same_result (& sealer , & opener , & key , & nonce , input , tag_size) ; seal_open_equals_expected (& sealer , & opener , & key , & nonce , input , expected_ct_with_tag , tag_size , aad ,) ; seal_plaintext_length (& sealer , & key , & nonce , tag_size , aad) ; open_ciphertext_with_tag_length (& sealer , & opener , & key , & nonce , tag_size , aad) ; }
    };
}

AeadTestRunner!()