macro_rules! deps {
    () => {
        Options!();
        MemtableFactory!();
        CompactionPri!();
        Cache!();
        DB!();
        WriteBufferManager!();
    };
}

macro_rules! tests {
    () => {
        deps!();
        # [cfg (test)] mod tests { use crate :: db_options :: WriteBufferManager ; use crate :: { Cache , CompactionPri , MemtableFactory , Options } ; # [test] fn test_enable_statistics () { let mut opts = Options :: default () ; opts . enable_statistics () ; opts . set_stats_dump_period_sec (60) ; assert ! (opts . get_statistics () . is_some ()) ; let opts = Options :: default () ; assert ! (opts . get_statistics () . is_none ()) ; } # [test] fn test_set_memtable_factory () { let mut opts = Options :: default () ; opts . set_memtable_factory (MemtableFactory :: Vector) ; opts . set_memtable_factory (MemtableFactory :: HashLinkList { bucket_count : 100 }) ; opts . set_memtable_factory (MemtableFactory :: HashSkipList { bucket_count : 100 , height : 4 , branching_factor : 4 , }) ; } # [test] fn test_use_fsync () { let mut opts = Options :: default () ; assert ! (! opts . get_use_fsync ()) ; opts . set_use_fsync (true) ; assert ! (opts . get_use_fsync ()) ; } # [test] fn test_set_stats_persist_period_sec () { let mut opts = Options :: default () ; opts . enable_statistics () ; opts . set_stats_persist_period_sec (5) ; assert ! (opts . get_statistics () . is_some ()) ; let opts = Options :: default () ; assert ! (opts . get_statistics () . is_none ()) ; } # [test] fn test_set_write_buffer_manager () { let mut opts = Options :: default () ; let lrucache = Cache :: new_lru_cache (100) ; let write_buffer_manager = WriteBufferManager :: new_write_buffer_manager_with_cache (100 , false , lrucache) ; assert_eq ! (write_buffer_manager . get_buffer_size () , 100) ; assert_eq ! (write_buffer_manager . get_usage () , 0) ; assert ! (write_buffer_manager . enabled ()) ; opts . set_write_buffer_manager (& write_buffer_manager) ; drop (opts) ; assert ! (write_buffer_manager . enabled ()) ; } # [test] fn compaction_pri () { let mut opts = Options :: default () ; opts . set_compaction_pri (CompactionPri :: RoundRobin) ; opts . create_if_missing (true) ; let tmp = tempfile :: tempdir () . unwrap () ; let _db = crate :: DB :: open (& opts , tmp . path ()) . unwrap () ; let options = std :: fs :: read_dir (tmp . path ()) . unwrap () . find_map (| x | { let x = x . ok () ? ; x . file_name () . into_string () . unwrap () . contains ("OPTIONS") . then_some (x . path ()) }) . map (std :: fs :: read_to_string) . unwrap () . unwrap () ; assert ! (options . contains ("compaction_pri=kRoundRobin")) ; } }
    };
}

tests!()