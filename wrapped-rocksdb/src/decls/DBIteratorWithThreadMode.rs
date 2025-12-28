macro_rules! deps {
    () => {
        DBAccess!();
        DBRawIteratorWithThreadMode!();
        Options!();
        Direction!();
        IteratorMode!();
    };
}

macro_rules! DBIteratorWithThreadMode {
    () => {
        deps!();
        # [doc = " A standard Rust [`Iterator`] over a database or column family."] # [doc = ""] # [doc = " As an alternative, [`DBRawIteratorWithThreadMode`] is a low level wrapper around"] # [doc = " RocksDB's API, which can provide better performance and more features."] # [doc = ""] # [doc = " ```"] # [doc = " use rocksdb::{DB, Direction, IteratorMode, Options};"] # [doc = ""] # [doc = " let tempdir = tempfile::Builder::new()"] # [doc = "     .prefix(\"_path_for_rocksdb_storage2\")"] # [doc = "     .tempdir()"] # [doc = "     .expect(\"Failed to create temporary path for the _path_for_rocksdb_storage2.\");"] # [doc = " let path = tempdir.path();"] # [doc = " {"] # [doc = "     let db = DB::open_default(path).unwrap();"] # [doc = "     let mut iter = db.iterator(IteratorMode::Start); // Always iterates forward"] # [doc = "     for item in iter {"] # [doc = "         let (key, value) = item.unwrap();"] # [doc = "         println!(\"Saw {:?} {:?}\", key, value);"] # [doc = "     }"] # [doc = "     iter = db.iterator(IteratorMode::End);  // Always iterates backward"] # [doc = "     for item in iter {"] # [doc = "         let (key, value) = item.unwrap();"] # [doc = "         println!(\"Saw {:?} {:?}\", key, value);"] # [doc = "     }"] # [doc = "     iter = db.iterator(IteratorMode::From(b\"my key\", Direction::Forward)); // From a key in Direction::{forward,reverse}"] # [doc = "     for item in iter {"] # [doc = "         let (key, value) = item.unwrap();"] # [doc = "         println!(\"Saw {:?} {:?}\", key, value);"] # [doc = "     }"] # [doc = ""] # [doc = "     // You can seek with an existing Iterator instance, too"] # [doc = "     iter = db.iterator(IteratorMode::Start);"] # [doc = "     iter.set_mode(IteratorMode::From(b\"another key\", Direction::Reverse));"] # [doc = "     for item in iter {"] # [doc = "         let (key, value) = item.unwrap();"] # [doc = "         println!(\"Saw {:?} {:?}\", key, value);"] # [doc = "     }"] # [doc = " }"] # [doc = " let _ = DB::destroy(&Options::default(), path);"] # [doc = " ```"] pub struct DBIteratorWithThreadMode < 'a , D : DBAccess > { raw : DBRawIteratorWithThreadMode < 'a , D > , direction : Direction , done : bool , }
    };
}

DBIteratorWithThreadMode!();