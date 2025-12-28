macro_rules! deps {
    () => {
        Error!();
        MemoryUsageBuilder!();
        DB!();
        MemoryUsageStats!();
        Cache!();
    };
}

macro_rules! get_memory_usage_stats {
    () => {
        deps!();
        # [doc = " Get memory usage stats from DB instances and Cache instances"] pub fn get_memory_usage_stats (dbs : Option < & [& DB] > , caches : Option < & [& Cache] > ,) -> Result < MemoryUsageStats , Error > { let mut builder = MemoryUsageBuilder :: new () ? ; if let Some (dbs_) = dbs { dbs_ . iter () . for_each (| db | builder . add_db (db)) ; } if let Some (caches_) = caches { caches_ . iter () . for_each (| cache | builder . add_cache (cache)) ; } let mu = builder . build () ? ; Ok (MemoryUsageStats { mem_table_total : mu . approximate_mem_table_total () , mem_table_unflushed : mu . approximate_mem_table_unflushed () , mem_table_readers_total : mu . approximate_mem_table_readers_total () , cache_total : mu . approximate_cache_total () , }) }
    };
}

get_memory_usage_stats!();