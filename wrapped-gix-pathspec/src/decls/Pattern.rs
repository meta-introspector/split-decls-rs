macro_rules! deps {
    () => {
        SearchMode!();
    };
}

macro_rules! Pattern {
    () => {
        deps!();
        # [doc = " The output of a pathspec [parsing][parse()] operation. It can be used to match against a one or more paths."] # [derive (Default , PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone)] pub struct Pattern { # [doc = " The path part of a pathspec, which is typically a path possibly mixed with glob patterns."] # [doc = " Note that it might be an empty string as well."] # [doc = ""] # [doc = " For example, `:(top,literal,icase,attr,exclude)some/path` would yield `some/path`."] path : BString , # [doc = " All magic signatures that were included in the pathspec."] pub signature : MagicSignature , # [doc = " The search mode of the pathspec."] pub search_mode : SearchMode , # [doc = " All attributes that were included in the `ATTR` part of the pathspec, if present."] # [doc = ""] # [doc = " `:(attr:a=one b=):path` would yield attribute `a` and `b`."] pub attributes : Vec < gix_attributes :: Assignment > , # [doc = " If `true`, we are a special Nil pattern and always match."] nil : bool , # [doc = " The length of bytes in `path` that belong to the prefix, which will always be matched case-sensitively"] # [doc = " on case-sensitive filesystems."] # [doc = ""] # [doc = " That way, even though pathspecs are applied from the top, we can emulate having changed directory into"] # [doc = " a specific sub-directory in a case-sensitive file-system, even if the rest of the pathspec can be set to"] # [doc = " match case-insensitively."] # [doc = " Is set by [Pattern::normalize()]."] prefix_len : usize , }
    };
}

Pattern!()