macro_rules! deps {
    () => {
        CompactionFilter!();
        CompactionFilterContext!();
    };
}

macro_rules! CompactionFilterFactory {
    () => {
        deps!();
        # [doc = " Each compaction will create a new CompactionFilter allowing the"] # [doc = " application to know about different compactions."] # [doc = ""] # [doc = " See [compaction_filter::CompactionFilter][CompactionFilter] and"] # [doc = " [Options::set_compaction_filter_factory][set_compaction_filter_factory]"] # [doc = " for more details"] # [doc = ""] # [doc = " [CompactionFilter]: ../compaction_filter/trait.CompactionFilter.html"] # [doc = " [set_compaction_filter_factory]: ../struct.Options.html#method.set_compaction_filter_factory"] pub trait CompactionFilterFactory { type Filter : CompactionFilter ; # [doc = " Returns a CompactionFilter for the compaction process"] fn create (& mut self , context : CompactionFilterContext) -> Self :: Filter ; # [doc = " Returns a name that identifies this compaction filter factory."] fn name (& self) -> & CStr ; }
    };
}

CompactionFilterFactory!()