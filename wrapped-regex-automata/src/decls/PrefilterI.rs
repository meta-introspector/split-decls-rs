macro_rules! deps {
    () => {
        Prefilter!();
        Span!();
    };
}

macro_rules! PrefilterI {
    () => {
        deps!();
        # [doc = " A trait for abstracting over prefilters. Basically, a prefilter is"] # [doc = " something that do an unanchored *and* an anchored search in a haystack"] # [doc = " within a given span."] # [doc = ""] # [doc = " This exists pretty much only so that we can use prefilters as a trait"] # [doc = " object (which is what `Prefilter` is). If we ever move off of trait objects"] # [doc = " and to an enum, then it's likely this trait could be removed."] pub (crate) trait PrefilterI : Debug + Send + Sync + RefUnwindSafe + UnwindSafe + 'static { # [doc = " Run this prefilter on `haystack[span.start..end]` and return a matching"] # [doc = " span if one exists."] # [doc = ""] # [doc = " The span returned is guaranteed to have a start position greater than"] # [doc = " or equal to the one given, and an end position less than or equal to"] # [doc = " the one given."] fn find (& self , haystack : & [u8] , span : Span) -> Option < Span > ; # [doc = " Returns the span of a prefix of `haystack[span.start..span.end]` if"] # [doc = " the prefilter matches."] # [doc = ""] # [doc = " The span returned is guaranteed to have a start position equivalent to"] # [doc = " the one given, and an end position less than or equal to the one given."] fn prefix (& self , haystack : & [u8] , span : Span) -> Option < Span > ; # [doc = " Returns the heap memory, in bytes, used by the underlying prefilter."] fn memory_usage (& self) -> usize ; # [doc = " Implementations might return true here if they believe themselves to"] # [doc = " be \"fast.\" See [`Prefilter::is_fast`] for more details."] fn is_fast (& self) -> bool ; }
    };
}

PrefilterI!()