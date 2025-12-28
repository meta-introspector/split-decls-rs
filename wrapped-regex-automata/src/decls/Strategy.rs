macro_rules! deps {
    () => {
        PatternSet!();
        HalfMatch!();
        Match!();
        Input!();
        Cache!();
        NonMaxUsize!();
        PatternID!();
        GroupInfo!();
    };
}

macro_rules! Strategy {
    () => {
        deps!();
        # [doc = " A trait that represents a single meta strategy. Its main utility is in"] # [doc = " providing a way to do dynamic dispatch over a few choices."] # [doc = ""] # [doc = " Why dynamic dispatch? I actually don't have a super compelling reason, and"] # [doc = " importantly, I have not benchmarked it with the main alternative: an enum."] # [doc = " I went with dynamic dispatch initially because the regex engine search code"] # [doc = " really can't be inlined into caller code in most cases because it's just"] # [doc = " too big. In other words, it is already expected that every regex search"] # [doc = " will entail at least the cost of a function call."] # [doc = ""] # [doc = " I do wonder whether using enums would result in better codegen overall"] # [doc = " though. It's a worthwhile experiment to try. Probably the most interesting"] # [doc = " benchmark to run in such a case would be one with a high match count. That"] # [doc = " is, a benchmark to test the overall latency of a search call."] pub (super) trait Strategy : Debug + Send + Sync + RefUnwindSafe + UnwindSafe + 'static { fn group_info (& self) -> & GroupInfo ; fn create_cache (& self) -> Cache ; fn reset_cache (& self , cache : & mut Cache) ; fn is_accelerated (& self) -> bool ; fn memory_usage (& self) -> usize ; fn search (& self , cache : & mut Cache , input : & Input < '_ >) -> Option < Match > ; fn search_half (& self , cache : & mut Cache , input : & Input < '_ > ,) -> Option < HalfMatch > ; fn is_match (& self , cache : & mut Cache , input : & Input < '_ >) -> bool ; fn search_slots (& self , cache : & mut Cache , input : & Input < '_ > , slots : & mut [Option < NonMaxUsize >] ,) -> Option < PatternID > ; fn which_overlapping_matches (& self , cache : & mut Cache , input : & Input < '_ > , patset : & mut PatternSet ,) ; }
    };
}

Strategy!();