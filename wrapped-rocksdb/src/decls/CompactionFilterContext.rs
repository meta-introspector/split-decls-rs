macro_rules! CompactionFilterContext {
    () => {
        # [doc = " Context information of a compaction run"] pub struct CompactionFilterContext { # [doc = " Does this compaction run include all data files"] pub is_full_compaction : bool , # [doc = " Is this compaction requested by the client (true),"] # [doc = " or is it occurring as an automatic compaction process"] pub is_manual_compaction : bool , }
    };
}

CompactionFilterContext!()