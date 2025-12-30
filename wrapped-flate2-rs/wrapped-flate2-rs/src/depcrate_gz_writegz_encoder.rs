// Generated macro for gz_encoder (function)
macro_rules! Depcrate_gz_writegz_encoder {
() => {
// Module: crate::gz::write
// Provides: {"gz_encoder"}
// Dependencies: {}
pub fn gz_encoder < W : Write > (header : Vec < u8 > , w : W , lvl : Compression) -> GzEncoder < W > { GzEncoder { inner : zio :: Writer :: new (w , Compress :: new (lvl , false)) , crc : Crc :: new () , header , crc_bytes_written : 0 , } }
};
}
