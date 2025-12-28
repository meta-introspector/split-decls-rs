macro_rules! deps {
    () => {
        Teddy!();
        ByteSet!();
        AhoCorasick!();
        Memchr2!();
        Memchr!();
        PrefilterI!();
        Strategy!();
        Memchr3!();
        Memmem!();
    };
}

macro_rules! Choice {
    () => {
        deps!();
        # [doc = " A type that encapsulates the selection of a prefilter algorithm from a"] # [doc = " sequence of needles."] # [doc = ""] # [doc = " The existence of this type is a little tricky, because we don't (currently)"] # [doc = " use it for performing a search. Instead, we really only consume it by"] # [doc = " converting the underlying prefilter into a trait object, whether that be"] # [doc = " `dyn PrefilterI` or `dyn Strategy` (for the meta regex engine). In order"] # [doc = " to avoid re-copying the prefilter selection logic, we isolate it here, and"] # [doc = " then force anything downstream that wants to convert it to a trait object"] # [doc = " to do trivial case analysis on it."] # [doc = ""] # [doc = " One wonders whether we *should* use an enum instead of a trait object."] # [doc = " At time of writing, I chose trait objects based on instinct because 1) I"] # [doc = " knew I wasn't going to inline anything and 2) there would potentially be"] # [doc = " many different choices. However, as of time of writing, I haven't actually"] # [doc = " compared the trait object approach to the enum approach. That probably"] # [doc = " should be litigated, but I ran out of steam."] # [doc = ""] # [doc = " Note that if the `alloc` feature is disabled, then values of this type"] # [doc = " are (and should) never be constructed. Also, in practice, for any of the"] # [doc = " prefilters to be selected, you'll need at least one of the `perf-literal-*`"] # [doc = " features enabled."] # [derive (Clone , Debug)] pub (crate) enum Choice { Memchr (Memchr) , Memchr2 (Memchr2) , Memchr3 (Memchr3) , Memmem (Memmem) , Teddy (Teddy) , ByteSet (ByteSet) , AhoCorasick (AhoCorasick) , }
    };
}

Choice!();