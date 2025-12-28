macro_rules! deps {
    () => {
        SstFileWriter!();
        Options!();
    };
}

macro_rules! IngestExternalFileOptions {
    () => {
        deps!();
        # [doc = " For configuring external files ingestion."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Move files instead of copying them:"] # [doc = ""] # [doc = " ```"] # [doc = " use rocksdb::{DB, IngestExternalFileOptions, SstFileWriter, Options};"] # [doc = ""] # [doc = " let writer_opts = Options::default();"] # [doc = " let mut writer = SstFileWriter::create(&writer_opts);"] # [doc = " let tempdir = tempfile::Builder::new()"] # [doc = "     .tempdir()"] # [doc = "     .expect(\"Failed to create temporary folder for the _path_for_sst_file\");"] # [doc = " let path1 = tempdir.path().join(\"_path_for_sst_file\");"] # [doc = " writer.open(path1.clone()).unwrap();"] # [doc = " writer.put(b\"k1\", b\"v1\").unwrap();"] # [doc = " writer.finish().unwrap();"] # [doc = ""] # [doc = " let tempdir2 = tempfile::Builder::new()"] # [doc = "     .prefix(\"_path_for_rocksdb_storageY3\")"] # [doc = "     .tempdir()"] # [doc = "     .expect(\"Failed to create temporary path for the _path_for_rocksdb_storageY3\");"] # [doc = " let path2 = tempdir2.path();"] # [doc = " {"] # [doc = "   let db = DB::open_default(&path2).unwrap();"] # [doc = "   let mut ingest_opts = IngestExternalFileOptions::default();"] # [doc = "   ingest_opts.set_move_files(true);"] # [doc = "   db.ingest_external_file_opts(&ingest_opts, vec![path1]).unwrap();"] # [doc = " }"] # [doc = " let _ = DB::destroy(&Options::default(), path2);"] # [doc = " ```"] pub struct IngestExternalFileOptions { pub (crate) inner : * mut ffi :: rocksdb_ingestexternalfileoptions_t , }
    };
}

IngestExternalFileOptions!()