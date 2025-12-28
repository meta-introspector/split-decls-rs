macro_rules! deps {
    () => {
        SearchMode!();
        Pattern!();
    };
}

macro_rules! Defaults {
    () => {
        deps!();
        # [doc = " Default settings for some fields of a [`Pattern`]."] # [doc = ""] # [doc = " These can be used to represent `GIT_*_PATHSPECS` environment variables, for example."] # [derive (Debug , Default , Copy , Clone , Ord , PartialOrd , Eq , PartialEq)] pub struct Defaults { # [doc = " The default signature."] pub signature : MagicSignature , # [doc = " The default search-mode."] # [doc = ""] # [doc = " Note that even if it's [`SearchMode::Literal`], the pathspecs will be parsed as usual, but matched verbatim afterwards."] # [doc = ""] # [doc = " Note that pathspecs can override this the [`SearchMode::Literal`] variant with an explicit `:(glob)` prefix."] pub search_mode : SearchMode , # [doc = " If set, the pathspec will not be parsed but used verbatim. Implies [`SearchMode::Literal`] for `search_mode`."] pub literal : bool , }
    };
}

Defaults!();