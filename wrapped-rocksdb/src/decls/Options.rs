macro_rules! deps {
    () => {
        OptionsMustOutliveDB!();
        DB!();
    };
}

macro_rules! Options {
    () => {
        deps!();
        # [doc = " Database-wide options around performance and behavior."] # [doc = ""] # [doc = " Please read the official tuning [guide](https://github.com/facebook/rocksdb/wiki/RocksDB-Tuning-Guide)"] # [doc = " and most importantly, measure performance under realistic workloads with realistic hardware."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use rocksdb::{Options, DB};"] # [doc = " use rocksdb::DBCompactionStyle;"] # [doc = ""] # [doc = " fn badly_tuned_for_somebody_elses_disk() -> DB {"] # [doc = "    let path = \"path/for/rocksdb/storageX\";"] # [doc = "    let mut opts = Options::default();"] # [doc = "    opts.create_if_missing(true);"] # [doc = "    opts.set_max_open_files(10000);"] # [doc = "    opts.set_use_fsync(false);"] # [doc = "    opts.set_bytes_per_sync(8388608);"] # [doc = "    opts.optimize_for_point_lookup(1024);"] # [doc = "    opts.set_table_cache_num_shard_bits(6);"] # [doc = "    opts.set_max_write_buffer_number(32);"] # [doc = "    opts.set_write_buffer_size(536870912);"] # [doc = "    opts.set_target_file_size_base(1073741824);"] # [doc = "    opts.set_min_write_buffer_number_to_merge(4);"] # [doc = "    opts.set_level_zero_stop_writes_trigger(2000);"] # [doc = "    opts.set_level_zero_slowdown_writes_trigger(0);"] # [doc = "    opts.set_compaction_style(DBCompactionStyle::Universal);"] # [doc = "    opts.set_disable_auto_compactions(true);"] # [doc = ""] # [doc = "    DB::open(&opts, path).unwrap()"] # [doc = " }"] # [doc = " ```"] pub struct Options { pub (crate) inner : * mut ffi :: rocksdb_options_t , pub (crate) outlive : OptionsMustOutliveDB , }
    };
}

Options!();