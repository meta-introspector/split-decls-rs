// Generated macro for use_4 (use)
macro_rules! Depcrate_recoveryuse_4 {
() => {
// Module: crate::recovery
// Provides: {"use_4"}
// Dependencies: {}
# [cfg (feature = "algorithm")] use { crate :: { EcdsaCurve , Signature , SignatureSize , SigningKey , VerifyingKey , hazmat :: { DigestAlgorithm , bits2field , sign_prehashed_rfc6979 , verify_prehashed } , } , digest :: { Digest , block_api :: EagerHash } , elliptic_curve :: { AffinePoint , FieldBytesEncoding , FieldBytesSize , Group , PrimeField , ProjectivePoint , bigint :: CheckedAdd , ops :: { LinearCombination , Reduce } , point :: DecompressPoint , sec1 :: { self , FromEncodedPoint , ToEncodedPoint } , } , elliptic_curve :: { CurveArithmetic , FieldBytes , Scalar , array :: ArraySize , ops :: Invert , subtle :: CtOption , } , signature :: { DigestSigner , MultipartSigner , RandomizedDigestSigner , Signer , hazmat :: { PrehashSigner , RandomizedPrehashSigner } , rand_core :: TryCryptoRng , } , } ;
};
}
