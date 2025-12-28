macro_rules! deps {
    () => {
        IngestExternalFileOptions!();
        DB!();
    };
}

macro_rules! impl_211 {
    () => {
        deps!();
        impl IngestExternalFileOptions { # [doc = " Can be set to true to move the files instead of copying them."] pub fn set_move_files (& mut self , v : bool) { unsafe { ffi :: rocksdb_ingestexternalfileoptions_set_move_files (self . inner , c_uchar :: from (v)) ; } } # [doc = " If set to false, an ingested file keys could appear in existing snapshots"] # [doc = " that where created before the file was ingested."] pub fn set_snapshot_consistency (& mut self , v : bool) { unsafe { ffi :: rocksdb_ingestexternalfileoptions_set_snapshot_consistency (self . inner , c_uchar :: from (v) ,) ; } } # [doc = " If set to false, IngestExternalFile() will fail if the file key range"] # [doc = " overlaps with existing keys or tombstones in the DB."] pub fn set_allow_global_seqno (& mut self , v : bool) { unsafe { ffi :: rocksdb_ingestexternalfileoptions_set_allow_global_seqno (self . inner , c_uchar :: from (v) ,) ; } } # [doc = " If set to false and the file key range overlaps with the memtable key range"] # [doc = " (memtable flush required), IngestExternalFile will fail."] pub fn set_allow_blocking_flush (& mut self , v : bool) { unsafe { ffi :: rocksdb_ingestexternalfileoptions_set_allow_blocking_flush (self . inner , c_uchar :: from (v) ,) ; } } # [doc = " Set to true if you would like duplicate keys in the file being ingested"] # [doc = " to be skipped rather than overwriting existing data under that key."] # [doc = " Usecase: back-fill of some historical data in the database without"] # [doc = " over-writing existing newer version of data."] # [doc = " This option could only be used if the DB has been running"] # [doc = " with allow_ingest_behind=true since the dawn of time."] # [doc = " All files will be ingested at the bottommost level with seqno=0."] pub fn set_ingest_behind (& mut self , v : bool) { unsafe { ffi :: rocksdb_ingestexternalfileoptions_set_ingest_behind (self . inner , c_uchar :: from (v)) ; } } }
    };
}

impl_211!();