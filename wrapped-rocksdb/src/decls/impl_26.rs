macro_rules! deps {
    () => {
        Error!();
        BackupEngineOptions!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl BackupEngineOptions { # [doc = " Initializes `BackupEngineOptions` with the directory to be used for storing/accessing the"] # [doc = " backup files."] pub fn new < P : AsRef < Path > > (backup_dir : P) -> Result < Self , Error > { let backup_dir = backup_dir . as_ref () ; let c_backup_dir = CString :: new (backup_dir . to_string_lossy () . as_bytes ()) . map_err (| _ | { Error :: new ("Failed to convert backup_dir to CString \
                     when constructing BackupEngineOptions" . to_owned () ,) }) ? ; unsafe { let opts = ffi :: rocksdb_backup_engine_options_create (c_backup_dir . as_ptr ()) ; assert ! (! opts . is_null () , "Could not create RocksDB backup options") ; Ok (Self { inner : opts }) } } # [doc = " Sets the number of operations (such as file copies or file checksums) that `RocksDB` may"] # [doc = " perform in parallel when executing a backup or restore."] # [doc = ""] # [doc = " Default: 1"] pub fn set_max_background_operations (& mut self , max_background_operations : i32) { unsafe { ffi :: rocksdb_backup_engine_options_set_max_background_operations (self . inner , max_background_operations ,) ; } } # [doc = " Sets whether to use fsync(2) to sync file data and metadata to disk after every file write,"] # [doc = " guaranteeing that backups will be consistent after a reboot or if machine crashes. Setting"] # [doc = " it to false will speed things up a bit, but some (newer) backups might be inconsistent. In"] # [doc = " most cases, everything should be fine, though."] # [doc = ""] # [doc = " Default: true"] # [doc = ""] # [doc = " Documentation: <https://github.com/facebook/rocksdb/wiki/How-to-backup-RocksDB#advanced-usage>"] pub fn set_sync (& mut self , sync : bool) { unsafe { ffi :: rocksdb_backup_engine_options_set_sync (self . inner , c_uchar :: from (sync)) ; } } # [doc = " Returns the value of the `sync` option."] pub fn get_sync (& mut self) -> bool { let val_u8 = unsafe { ffi :: rocksdb_backup_engine_options_get_sync (self . inner) } ; val_u8 != 0 } }
    };
}

impl_26!()