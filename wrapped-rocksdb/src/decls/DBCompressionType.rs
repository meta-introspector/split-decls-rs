macro_rules! DBCompressionType {
    () => {
        # [derive (Debug , Copy , Clone , PartialEq , Eq)] # [cfg_attr (feature = "serde1" , derive (serde :: Serialize , serde :: Deserialize))] pub enum DBCompressionType { None = ffi :: rocksdb_no_compression as isize , Snappy = ffi :: rocksdb_snappy_compression as isize , Zlib = ffi :: rocksdb_zlib_compression as isize , Bz2 = ffi :: rocksdb_bz2_compression as isize , Lz4 = ffi :: rocksdb_lz4_compression as isize , Lz4hc = ffi :: rocksdb_lz4hc_compression as isize , Zstd = ffi :: rocksdb_zstd_compression as isize , }
    };
}

DBCompressionType!()