macro_rules! deps {
    () => {
        CheckType!();
        ChecksumCalculator!();
    };
}

macro_rules! impl_157 {
    () => {
        deps!();
        impl ChecksumCalculator { fn new (check_type : CheckType) -> Self { match check_type { CheckType :: None => Self :: None , CheckType :: Crc32 => Self :: Crc32 (CRC32 . digest ()) , CheckType :: Crc64 => Self :: Crc64 (CRC64 . digest ()) , CheckType :: Sha256 => Self :: Sha256 (sha2 :: Sha256 :: new ()) , } } fn update (& mut self , data : & [u8]) { match self { ChecksumCalculator :: None => { } ChecksumCalculator :: Crc32 (crc) => { crc . update (data) ; } ChecksumCalculator :: Crc64 (crc) => { crc . update (data) ; } ChecksumCalculator :: Sha256 (sha) => { sha . update (data) ; } } } fn verify (self , expected : & [u8]) -> bool { match self { ChecksumCalculator :: None => true , ChecksumCalculator :: Crc32 (crc) => { if expected . len () != 4 { return false ; } let expected_crc = u32 :: from_le_bytes ([expected [0] , expected [1] , expected [2] , expected [3]]) ; let final_crc = crc . finalize () ; final_crc == expected_crc } ChecksumCalculator :: Crc64 (crc) => { if expected . len () != 8 { return false ; } let expected_crc = u64 :: from_le_bytes ([expected [0] , expected [1] , expected [2] , expected [3] , expected [4] , expected [5] , expected [6] , expected [7] ,]) ; let final_crc = crc . finalize () ; final_crc == expected_crc } ChecksumCalculator :: Sha256 (sha) => { if expected . len () != 32 { return false ; } let final_sha = sha . finalize () ; & final_sha [.. 32] == expected } } } # [cfg (feature = "encoder")] fn finalize_to_bytes (self) -> Vec < u8 > { match self { ChecksumCalculator :: None => Vec :: new () , ChecksumCalculator :: Crc32 (crc) => crc . finalize () . to_le_bytes () . to_vec () , ChecksumCalculator :: Crc64 (crc) => crc . finalize () . to_le_bytes () . to_vec () , ChecksumCalculator :: Sha256 (sha) => sha . finalize () . to_vec () , } } }
    };
}

impl_157!();