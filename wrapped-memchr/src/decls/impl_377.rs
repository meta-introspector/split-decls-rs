macro_rules! deps {
    () => {
        DefaultFrequencyRank!();
        FinderRev!();
        HeuristicFrequencyRank!();
        Prefilter!();
        Searcher!();
        CowBytes!();
        Finder!();
        SearcherRev!();
        FinderBuilder!();
    };
}

macro_rules! impl_377 {
    () => {
        deps!();
        impl FinderBuilder { # [doc = " Create a new finder builder with default settings."] pub fn new () -> FinderBuilder { FinderBuilder :: default () } # [doc = " Build a forward finder using the given needle from the current"] # [doc = " settings."] pub fn build_forward < 'n , B : ? Sized + AsRef < [u8] > > (& self , needle : & 'n B ,) -> Finder < 'n > { self . build_forward_with_ranker (DefaultFrequencyRank , needle) } # [doc = " Build a forward finder using the given needle and a custom heuristic for"] # [doc = " determining the frequency of a given byte in the dataset."] # [doc = " See [`HeuristicFrequencyRank`] for more details."] pub fn build_forward_with_ranker < 'n , R : HeuristicFrequencyRank , B : ? Sized + AsRef < [u8] > , > (& self , ranker : R , needle : & 'n B ,) -> Finder < 'n > { let needle = needle . as_ref () ; Finder { needle : CowBytes :: new (needle) , searcher : Searcher :: new (self . prefilter , ranker , needle) , } } # [doc = " Build a reverse finder using the given needle from the current"] # [doc = " settings."] pub fn build_reverse < 'n , B : ? Sized + AsRef < [u8] > > (& self , needle : & 'n B ,) -> FinderRev < 'n > { let needle = needle . as_ref () ; FinderRev { needle : CowBytes :: new (needle) , searcher : SearcherRev :: new (needle) , } } # [doc = " Configure the prefilter setting for the finder."] # [doc = ""] # [doc = " See the documentation for [`Prefilter`] for more discussion on why"] # [doc = " you might want to configure this."] pub fn prefilter (& mut self , prefilter : Prefilter) -> & mut FinderBuilder { self . prefilter = prefilter ; self } }
    };
}

impl_377!()